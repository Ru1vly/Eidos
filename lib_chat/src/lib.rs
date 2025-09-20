pub struct Chat;

impl Chat {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) {
        println!("Chat is running");
    }
}

impl Default for Chat {
    fn default() -> Self {
        Self::new()
    }
}