// lib_chat/src/api.rs
use crate::error::{ChatError, Result};
use crate::history::Message;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

// Default timeouts (can be overridden via environment variables)
const DEFAULT_REQUEST_TIMEOUT_SECS: u64 = 30;
const DEFAULT_CONNECT_TIMEOUT_SECS: u64 = 10;

#[derive(Debug, Clone)]
pub enum ApiProvider {
    OpenAI {
        api_key: String,
        model: String,
    },
    Ollama {
        base_url: String,
        model: String,
    },
    Custom {
        base_url: String,
        api_key: Option<String>,
        model: String,
    },
}

impl ApiProvider {
    /// Load provider from environment variables
    /// Priority: OPENAI_API_KEY > OLLAMA_HOST > Custom
    pub fn from_env() -> Result<Self> {
        // Try OpenAI first
        if let Ok(api_key) = env::var("OPENAI_API_KEY") {
            let model = env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string());
            return Ok(ApiProvider::OpenAI { api_key, model });
        }

        // Try Ollama
        if let Ok(host) = env::var("OLLAMA_HOST") {
            let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| "llama2".to_string());
            return Ok(ApiProvider::Ollama {
                base_url: host,
                model,
            });
        }

        // Try custom provider
        if let Ok(base_url) = env::var("LLM_API_URL") {
            let api_key = env::var("LLM_API_KEY").ok();
            let model = env::var("LLM_MODEL").unwrap_or_else(|_| "default".to_string());
            return Ok(ApiProvider::Custom {
                base_url,
                api_key,
                model,
            });
        }

        Err(ChatError::NoProviderError)
    }

    pub fn model_name(&self) -> &str {
        match self {
            ApiProvider::OpenAI { model, .. } => model,
            ApiProvider::Ollama { model, .. } => model,
            ApiProvider::Custom { model, .. } => model,
        }
    }
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    message: ResponseMessage,
}

pub struct ApiClient {
    provider: ApiProvider,
    client: Client,
}

impl ApiClient {
    pub fn new(provider: ApiProvider) -> Result<Self> {
        // Get timeout values from environment variables or use defaults
        let request_timeout = env::var("HTTP_REQUEST_TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(DEFAULT_REQUEST_TIMEOUT_SECS);

        let connect_timeout = env::var("HTTP_CONNECT_TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(DEFAULT_CONNECT_TIMEOUT_SECS);

        // Create HTTP client with configurable timeouts to prevent hanging requests
        let client = Client::builder()
            .timeout(Duration::from_secs(request_timeout))
            .connect_timeout(Duration::from_secs(connect_timeout))
            .build()
            .map_err(|e| ChatError::ApiError(format!("Failed to build HTTP client: {}", e)))?;

        Ok(Self { provider, client })
    }

    pub fn from_env() -> Result<Self> {
        let provider = ApiProvider::from_env()?;
        Self::new(provider)
    }

    pub async fn send_message(
        &self,
        messages: &[Message],
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> Result<String> {
        match &self.provider {
            ApiProvider::OpenAI { api_key, model } => {
                self.send_openai_request(api_key, model, messages, temperature, max_tokens)
                    .await
            }
            ApiProvider::Ollama { base_url, model } => {
                self.send_ollama_request(base_url, model, messages).await
            }
            ApiProvider::Custom {
                base_url,
                api_key,
                model,
            } => {
                self.send_custom_request(
                    base_url,
                    api_key.as_deref(),
                    model,
                    messages,
                    temperature,
                    max_tokens,
                )
                .await
            }
        }
    }

    async fn send_openai_request(
        &self,
        api_key: &str,
        model: &str,
        messages: &[Message],
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> Result<String> {
        let url = "https://api.openai.com/v1/chat/completions";

        let request_body = OpenAIRequest {
            model: model.to_string(),
            messages: messages.to_vec(),
            temperature,
            max_tokens,
        };

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(ChatError::ApiError(format!(
                "API request failed with status {}: {}",
                status, error_text
            )));
        }

        let response_data: OpenAIResponse = response.json().await?;

        response_data
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| ChatError::InvalidResponse("No choices in response".to_string()))
    }

    async fn send_ollama_request(
        &self,
        base_url: &str,
        model: &str,
        messages: &[Message],
    ) -> Result<String> {
        let url = format!("{}/api/chat", base_url);

        let request_body = OllamaRequest {
            model: model.to_string(),
            messages: messages.to_vec(),
            stream: false,
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(ChatError::ApiError(format!(
                "Ollama API request failed with status {}: {}",
                status, error_text
            )));
        }

        let response_data: OllamaResponse = response.json().await?;
        Ok(response_data.message.content)
    }

    async fn send_custom_request(
        &self,
        base_url: &str,
        api_key: Option<&str>,
        model: &str,
        messages: &[Message],
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> Result<String> {
        let url = format!("{}/chat/completions", base_url);

        let request_body = OpenAIRequest {
            model: model.to_string(),
            messages: messages.to_vec(),
            temperature,
            max_tokens,
        };

        let mut request = self
            .client
            .post(&url)
            .header("Content-Type", "application/json");

        if let Some(key) = api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.json(&request_body).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(ChatError::ApiError(format!(
                "Custom API request failed with status {}: {}",
                status, error_text
            )));
        }

        let response_data: OpenAIResponse = response.json().await?;

        response_data
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| ChatError::InvalidResponse("No choices in response".to_string()))
    }
}
