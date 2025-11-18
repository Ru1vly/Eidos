use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Request {
    Chat,
    Core,
    Translate,
}

/// Handler function that takes input text and returns a Result
pub type Handler = Box<dyn Fn(&str) -> Result<(), String>>;

pub struct Bridge {
    router: HashMap<Request, Handler>,
}

impl Bridge {
    pub fn new() -> Self {
        Self {
            router: HashMap::new(),
        }
    }

    /// Register a handler for a specific request type
    pub fn register(&mut self, request: Request, handler: Handler) {
        self.router.insert(request, handler);
    }

    /// Route a request to its registered handler with input
    pub fn route(&self, request: Request, input: &str) -> Result<(), String> {
        if let Some(handler) = self.router.get(&request) {
            handler(input)
        } else {
            Err(format!("No handler registered for request: {:?}", request))
        }
    }
}

impl Default for Bridge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_new() {
        let bridge = Bridge::new();
        assert_eq!(bridge.router.len(), 0);
    }

    #[test]
    fn test_bridge_default() {
        let bridge = Bridge::default();
        assert_eq!(bridge.router.len(), 0);
    }

    #[test]
    fn test_register_handler() {
        let mut bridge = Bridge::new();

        bridge.register(Request::Chat, Box::new(|_text: &str| Ok(())));

        assert_eq!(bridge.router.len(), 1);
    }

    #[test]
    fn test_route_success() {
        let mut bridge = Bridge::new();

        // Create a handler that captures input
        bridge.register(
            Request::Chat,
            Box::new(|text: &str| {
                if text == "test" {
                    Ok(())
                } else {
                    Err("Unexpected input".to_string())
                }
            }),
        );

        // Test successful routing
        let result = bridge.route(Request::Chat, "test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_handler_error() {
        let mut bridge = Bridge::new();

        bridge.register(
            Request::Chat,
            Box::new(|_text: &str| Err("Handler error".to_string())),
        );

        let result = bridge.route(Request::Chat, "test");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Handler error");
    }

    #[test]
    fn test_route_no_handler() {
        let bridge = Bridge::new();

        let result = bridge.route(Request::Chat, "test");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No handler registered"));
    }

    #[test]
    fn test_multiple_handlers() {
        let mut bridge = Bridge::new();

        bridge.register(Request::Chat, Box::new(|_: &str| Ok(())));

        bridge.register(Request::Core, Box::new(|_: &str| Ok(())));

        bridge.register(Request::Translate, Box::new(|_: &str| Ok(())));

        assert_eq!(bridge.router.len(), 3);

        // All routes should work
        assert!(bridge.route(Request::Chat, "test").is_ok());
        assert!(bridge.route(Request::Core, "test").is_ok());
        assert!(bridge.route(Request::Translate, "test").is_ok());
    }

    #[test]
    fn test_handler_receives_input() {
        let mut bridge = Bridge::new();

        bridge.register(
            Request::Chat,
            Box::new(|text: &str| {
                // Verify the handler receives the correct input
                assert_eq!(text, "hello world");
                Ok(())
            }),
        );

        let result = bridge.route(Request::Chat, "hello world");
        assert!(result.is_ok());
    }

    #[test]
    fn test_request_enum_values() {
        // Test that all Request variants are distinct
        let chat = Request::Chat;
        let core = Request::Core;
        let translate = Request::Translate;

        assert_ne!(chat, core);
        assert_ne!(chat, translate);
        assert_ne!(core, translate);
    }

    #[test]
    fn test_overwrite_handler() {
        let mut bridge = Bridge::new();

        // Register first handler
        bridge.register(
            Request::Chat,
            Box::new(|_: &str| Err("First handler".to_string())),
        );

        // Overwrite with second handler
        bridge.register(Request::Chat, Box::new(|_: &str| Ok(())));

        // Should use the second handler
        let result = bridge.route(Request::Chat, "test");
        assert!(result.is_ok());
    }
}
