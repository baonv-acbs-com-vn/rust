use rb_lib_hub::{user::User, Hub, Vote, user::Player};

const COUNT: u64 = 1000000000;

fn main() {
    let mut hub: Hub = Hub::new();

    let mut count_down: bool = true;

    let mut count_down_count: u64 = 0;

    let mut user: User = User::new("user1", 1);

    let mut user2: User = User::new("user2", 2);

    // get vote from user input
    setup_vote(&mut hub, &mut user);

    setup_vote(&mut hub, &mut user2);
    
    while count_down {

        count_down_count += 1;
        count_down = false;

        for i in 0..COUNT { 
       
            if i == COUNT - 1 {
                if hub.list.len() > 1  {
                    hub.run();
                    println!("\nhub result {:?}", hub.result);
                    println!("\nhub list {:?}", hub.list);
                    if hub.list.len() > 1 && hub.list.contains(&user) {
                        println!("user {:?} is in the list", user);
                        setup_vote(&mut hub, &mut user);
                    }
                }
                println!("{}...", count_down_count);
                count_down = true;
            }
        }
    }

    println!("end");

}


pub fn get_vote() -> Vote {
    use std::io::{stdin, stdout, Write};
    loop {
        let mut s = String::new();
        print!("Please enter text A, B, or O: ");
        let _ = stdout().flush();

        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }

        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
    
    
        match s.to_uppercase().as_str() {
            "A" => return Vote::A,
            "B" => return Vote::B,
            "O" => return Vote::O,
            _ => {
                println!("Invalid input, please try again.");
            }
        }
    }
}


fn setup_vote(hub: &mut Hub, user: &mut User) {

    let vote = get_vote();

    println!("checking...");

    user.vote(vote);
    println!("{} typed: {:?}", user.name, user.vote);
    let _ = hub.add(user);
}