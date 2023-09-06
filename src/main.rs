use rand::Rng;
use std::io;
use std::cmp::Ordering;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

// add quit functionality
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    if cfg!(debug_assertions) {
        println!("Debug only: The secret number is: {secret_number}");
    }

    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        
        if guess.trim() == "quit" {
            break
        }
        
        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => {
                println!("Input must be a number between 1 and 100.");
                continue
            },
        };
        
        println!("You guessed: {}", guess.value);
    
        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}