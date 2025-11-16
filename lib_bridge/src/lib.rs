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
