use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Request {
    Chat,
    Core,
    Translate,
}

pub struct Bridge {
    router: HashMap<Request, Box<dyn Fn()>>,
}

impl Bridge {
    pub fn new() -> Self {
        Self {
            router: HashMap::new(),
        }
    }

    pub fn register(&mut self, request: Request, f: Box<dyn Fn()>) {
        self.router.insert(request, f);
    }

    pub fn route(&self, request: Request) {
        if let Some(f) = self.router.get(&request) {
            f();
        }
    }
}

impl Default for Bridge {
    fn default() -> Self {
        Self::new()
    }
}
