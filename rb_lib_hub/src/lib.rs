use user::{User, Player};

pub mod user;

#[derive(Debug,PartialEq,Clone)]
pub enum Vote {
    B,
    A,
    O
}

#[derive(Debug,PartialEq)]
pub struct Hub {
    pub list: Vec<User>,
    pub result: Option<Vote>,
}

impl Hub {

    pub fn new() -> Self {
        Self { list: Vec::new(), result: None}
    }

    pub fn add<'u>(&mut self, user: &'u mut User) -> &'u mut User {
        self.list.retain(|item| item.id != user.id);
        self.list.push(user.clone());
        user
    }

    pub fn run(&mut self) -> &mut Hub {
     
        let mut count_reult_a = 0;
        let mut count_reult_b = 0;
        let mut count_reult_o = 0;
        
      for i in &mut self.list {

        if i.vote == Some(Vote::A) {
            count_reult_a += 1;
        } else if i.vote == Some(Vote::B) {
            count_reult_b += 1;
        } else if i.vote == Some(Vote::O) {
                count_reult_o += 1;
            }
        }

        if count_reult_a > count_reult_b && count_reult_a > count_reult_o {
              self.result = Some(Vote::A)
        } else if count_reult_b > count_reult_a && count_reult_b > count_reult_o {
             self.result = Some(Vote::B)
        } else {
             self.result = Some(Vote::O)
        }

        self.list.retain(|item| item.vote == self.result);

        return self;
        
    }  

     
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hub_voting() {
        let mut hub: Hub = Hub::new();
        
        // Create users and add them to the hub
        let mut user1 = User::new("John", 1);
        user1.vote = Some(Vote::A);
        hub.add(&mut user1);

        let mut user2 = User::new("user1", 2);
        user2.vote = Some(Vote::A);
        hub.add(&mut user2);

        let mut user3 = User::new("user2", 3);
        user3.vote = Some(Vote::B);
        hub.add(&mut user3);

        // Run the hub to calculate the result
        hub.run();

        // Check if the result is as expected
        assert_eq!(hub.result, Some(Vote::A));

        // Check if the list contains only users with the winning vote
        assert_eq!(hub.list.len(), 2);
        assert!(hub.list.iter().all(|user| user.vote == Some(Vote::A)));
    }


    #[test]
    fn change_vote() {
        let mut hub: Hub = Hub::new();
        let mut user1 = User::new("user1", 1);
        user1.vote(Vote::A);
        hub.add(&mut user1);
        hub.run();
        user1.vote(Vote::B);
        hub.add(&mut user1);
        hub.run();
        assert_eq!(hub.list.len(), 1);
        assert_eq!(hub.result, Some(Vote::B));
    }
}
