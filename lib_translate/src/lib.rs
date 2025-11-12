pub struct Translate;

impl Translate {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, text: &str) {
        println!("Translate is running with text: {}", text);
    }
}

impl Default for Translate {
    fn default() -> Self {
        Self::new()
    }
}