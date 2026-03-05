use std::ops::Drop;
pub struct Tracker {
    pub name: String,
}

impl Tracker {
    pub fn new(name: String) -> Self {
        println!("Created tracker for `{}`", name);
        Self { name }
    }
}

impl Drop for Tracker {
    fn drop(&mut self) {
        println!("Dropping tracker for `{}`", self.name);
    }
}
