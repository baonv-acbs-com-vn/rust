use uuid::Uuid;

use crate::Vote;

#[derive(Debug,PartialEq, Clone)]
pub struct User {
    pub name: &'static str,
    pub id: Uuid,
    pub vote: Option<Vote>,
}

pub trait Player {
     fn vote(&mut self, vote: Vote);
}

impl Player for User {
     fn vote(&mut self, vote: Vote) {
        self.vote = Some(vote);
    }
   
}

impl User {
    pub fn new(name: &'static str) -> User {
        User { name, id: Uuid::new_v4(), vote: None }
    }
}
