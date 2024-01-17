// Defining that the project uses external crate(library/code)
extern crate rand;

//Importing all the necessary modules
//rand isnt a part of the standard library so add it in cargo.toml so build it
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let game_over = String::from("quit");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number: {}", secret_number);
    loop {
        println!("Guess the number!");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please Type a number!");
        println!("Guess Value: {}", guess);
        match guess {
            _ if guess == "quit" => break,
            _ => println!("Unexpected Error"),
        }
        if guess.eq(&game_over) {
            println!("Hello World!");
            break;
        }
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!!!!");
                break;
            }
        }
    }
}
