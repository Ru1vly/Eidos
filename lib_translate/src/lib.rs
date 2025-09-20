pub struct Translate;

impl Translate {
    pub fn new() -> Self {
        Self
    }

- [ ]     pub fn run(&self) {
        println!("Translate is running");
    }
}

impl Default for Translate {
    fn default() -> Self {
        Self::new()
    }
}