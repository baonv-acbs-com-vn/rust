use crate::Vote;

#[derive(Debug,PartialEq, Clone)]
pub struct User {
    pub name: &'static str,
    pub id: u64,
    pub vote: Option<Vote>
}

pub trait Player {
     fn vote(&mut self, vote: Vote);
     fn new(name: &'static str, id: u64) -> User;

}

impl Player for User {
     fn vote(&mut self, vote: Vote) {
        self.vote = Some(vote);
    }

     fn new(name: &'static str, id: u64) -> User {
        User { name, id, vote: None }
    }
}