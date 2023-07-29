// use std::net::IpAddr;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: i32,
}

fn main() {
    // let home: IpAddr = "127.0.0.1"
    //     .parse()
    //     .expect("Hardcoded IP address should be valid");

    println!("Guess the number!");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=10);

        println!("The secret number is: {secret_number}");

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        println!("You guessed: {}", guess.value);

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 10 {
            panic!("Guess value must be between 1 and 10, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
