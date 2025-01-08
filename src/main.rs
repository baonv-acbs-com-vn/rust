use rb_lib_hub::{user::User, Hub, Vote};

fn main() {
    let user: User = User::new("user1", 1);
    
    let mut hub: Hub = Hub::new();
    let _hub: Result<bool, &'static str >  = hub.add(user);
    
    match _hub {
        Ok(_) => {
             hub.vote(Vote::A);
        },
        Err(e) => println!("{} Error: ", e)
    }

    let user2: User = User::new("user2", 2);
    let _hub: Result<bool, &'static str >  = hub.add(user2);

    match _hub {
        Ok(_) => {
            hub.vote(Vote::B);
        },
        Err(e) => println!("{} Error: ", e)
    }

    let user3: User = User::new("user2", 3);
    let _hub: Result<bool, &'static str > = hub.add(user3);

    match _hub {
        Ok(_) => {
            hub.vote(Vote::B);
        },
        Err(e) => println!("{} Error: ", e)
    }

    hub.run();

    println!("\nhub result {:?}", hub.result);
}