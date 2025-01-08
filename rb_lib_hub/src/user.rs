#[derive(Debug,PartialEq)]
pub struct User {
    pub name: &'static str,
    pub id: u64
}

impl User {
    pub fn new(name: &'static str, id: u64) -> Self {
        Self { name, id }
    }
}
