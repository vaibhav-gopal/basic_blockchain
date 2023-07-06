use std::{
    io::{self, Write},
    process
};

use basic_blockchain::*;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Input a miner address: ");
    io::stdout().flush().expect("Could not flush stdout");
    io::stdin().read_line(&mut miner_addr).expect("Could not read miner address from stdin");

    print!("Input a difficulty: ");
    io::stdout().flush().expect("Could not flush stdout");
    io::stdin().read_line(&mut difficulty).expect("Could not read difficulty from stdin");
    let diff = difficulty.trim().parse::<u32>().expect("Please use integers for difficulty value");

    println!("Generating genesis block! ");
    let mut chain = Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine Block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        io::stdout().flush().expect("Could not flush stdout");
        choice.clear();
        io::stdin().read_line(&mut choice).expect("Could not read menu choice from stdin");
        println!("");

        if let Ok(ch) = choice.trim().parse() {
            match ch {
                1 => {
                    let mut sender = String::new();
                    let mut reciever = String::new();
                    let mut amount = String::new(); 
                    
                    print!("Input sender address: ");
                    io::stdout().flush().expect("Could not flush stdout");
                    io::stdin().read_line(&mut sender).expect("Could not read sender address from stdin");
                    sender = sender.trim().to_string();
                    print!("Input reciever address: ");
                    io::stdout().flush().expect("Could not flush stdout");
                    io::stdin().read_line(&mut reciever).expect("Could not read reciever address from stdin");
                    reciever = reciever.trim().to_string();
                    print!("Input an amount: ");
                    io::stdout().flush().expect("Could not flush stdout");
                    io::stdin().read_line(&mut amount).expect("Could not read amount from stdin");
                    let amount = match amount.trim().parse::<f32>() {
                        Ok(am) => am,
                        Err(_) => {
                            println!("Enter valid amount number!");
                            continue;
                        }
                    };

                    let res = chain.new_transaction(sender, reciever, amount);

                    match res {
                        true => println!("transaction added"),
                        false => println!("transaction failed")
                    }
                },
                2 => {
                    println!("Generating block...");
                    let res = chain.generate_new_block();
                    match res {
                        true => println!("Block generated succesfully"),
                        false => println!("Block generation failed...")
                    }
                },
                3 => {
                    let mut new_diff = String::new();
                    print!("Enter new difficulty: ");
                    io::stdout().flush().expect("Could not flush stdout");
                    io::stdin().read_line(&mut new_diff).expect("Could not read new difficulty from stdin");
                    let new_diff = match new_diff.trim().parse::<u32>() {
                        Ok(newdf) => newdf,
                        Err(_) => {
                            println!("Enter valid new difficulty!");
                            continue;
                        }
                    };
                    let res = chain.update_difficulty(new_diff);
                    match res {
                        true => println!("Updated difficulty"),
                        false => println!("Failed Updating Difficulty...")
                    }
                },
                4 => {
                    let mut new_reward = String::new();
                    print!("Enter new reward: ");
                    io::stdout().flush().expect("Could not flush stdout");
                    io::stdin().read_line(&mut new_reward).expect("Could not read new reward from stdin");
                    let new_reward = match new_reward.trim().parse::<u32>() {
                        Ok(newrw) => newrw,
                        Err(_) => {
                            println!("Enter valid new reward!");
                            continue;
                        }
                    };
                    let res = chain.update_difficulty(new_reward);
                    match res {
                        true => println!("Updated reward"),
                        false => println!("Failed Updating Reward...")
                    }
                },
                0 => {
                    println!("exiting!");
                    process::exit(0);
                },
                _ => {
                    println!("Enter a menu choice number within range!");
                    continue;
                },
            }
        } else {
            println!("Please enter a valid integer for menu choice!");
            continue;
        }
    }
}