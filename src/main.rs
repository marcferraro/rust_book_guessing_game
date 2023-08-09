use rand::Rng;
use std::io;
use std::cmp::Ordering;

// add quit functionality
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    if cfg!(debug_assertions) {
        println!("The secret number is: {secret_number}");
    }

    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // could add match based error handler
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input must be a number.");
                continue
            },
        };
        
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break
            },
        }
    }
}