use user::User;
pub mod user;

#[derive(Debug,PartialEq)]
pub enum Vote {
    B,
    A,
    O
}

#[derive(Debug,PartialEq)]
pub struct Hub {
    pub list: Vec<User>,
    pub result: Option<Vote>,
    pub votes: Vec<Vote>
}

impl Hub {

    pub fn new() -> Self {
        Self { list: Vec::new(), result: None, votes: Vec::new()}
    }

    pub fn add(&mut self, user: User) -> Result< bool, &'static str > {
        
        self.list.push(user);
      
        println!("\n {:?}", self.list);

        Ok(true)
    }

    pub fn vote(&mut self, input: Vote) -> &mut Hub { 
        self.votes.push(input);
       return   self;
    }

    pub fn run(&mut self) -> &mut Hub {
        let mut count_reult_a = 0;
        let mut count_reult_b = 0;
        let mut count_reult_o = 0;
        
      for i in &mut self.votes {

        if *i == Vote::A {
            count_reult_a += 1;
        } else if *i == Vote::B {
            count_reult_b += 1;
        } else if *i == Vote::O {
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

        return self;
        
    }   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hub_init() {
        let mut hub: Hub = Hub::new();
        let user: User = User::new("John", 1);
        let _hub: Result<bool, &'static str > = hub.add(user);
        hub.vote(Vote::A);
        let user1: User = User::new("user1", 1);
        let _hub: Result<bool, &'static str >  = hub.add(user1);
        hub.vote(Vote::A);
        let user2: User = User::new("user2", 1);
        let _hub: Result<bool, &'static str >  = hub.add(user2);
        hub.vote(Vote::B);
        hub.run();
        assert_eq!(hub.result, Some(Vote::A));
    }
}