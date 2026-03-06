pub struct Tracker {
    pub name: String,
    pub count: u8,
}

impl Tracker {
    pub fn new(name: String) -> Self {
        let count = 0;
        println!("created tracker {}. Count: {}", name, count);
        Self { name, count }
    }
}
