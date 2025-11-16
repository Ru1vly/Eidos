pub struct Chat;

impl Chat {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, text: &str) {
        println!("Chat is running with text: {}", text);
    }
}

impl Default for Chat {
    fn default() -> Self {
        Self::new()
    }
}
