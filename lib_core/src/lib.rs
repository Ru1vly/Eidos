pub struct Core;

impl Core {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) {
        println!("Core is running");
    }
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}