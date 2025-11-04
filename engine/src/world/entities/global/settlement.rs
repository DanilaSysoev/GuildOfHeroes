pub struct Settlement {
    name: String,
}

impl Settlement {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
