use rb_lib_hub::{user::User, Hub, Vote, user::Player};

use std::{
    error::Error,
    net::{Ipv4Addr, Ipv6Addr},
};

use clap::Parser;
use futures::StreamExt;
use libp2p::{
    core::{multiaddr::Protocol, Multiaddr},
    identify, identity, noise, ping, relay,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux,
};
use tracing_subscriber::EnvFilter;
// pub mod network;
const COUNT: u64 = 1000000000;

fn main() {
    let mut hub: Hub = Hub::new();

    let mut count_down: bool = true;

    let mut count_down_count: u64 = 0;

    let mut user: User = User::new("user1");

    let mut user2: User = User::new("user2");

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


#[derive(NetworkBehaviour)]
struct Behaviour {
    relay: relay::Behaviour,
    ping: ping::Behaviour,
    identify: identify::Behaviour,
}

fn generate_ed25519(secret_key_seed: u8) -> identity::Keypair {
    let mut bytes = [0u8; 32];
    bytes[0] = secret_key_seed;

    identity::Keypair::ed25519_from_bytes(bytes).expect("only errors on wrong length")
}

#[derive(Debug, Parser)]
#[clap(name = "libp2p relay")]
struct Opt {
    /// Determine if the relay listen on ipv6 or ipv4 loopback address. the default is ipv4
    #[clap(long)]
    use_ipv6: Option<bool>,

    /// Fixed value to generate deterministic peer id
    #[clap(long)]
    secret_key_seed: u8,

    /// The port used to listen on all interfaces
    #[clap(long)]
    port: u16,
}