use rand::Rng;
use std::io::{self, Write};

fn main() {
    let mut play_again: bool = true;

    while play_again {
        let choice: u8 = start_game();

        match choice {
            1 => {
                println!("Great! You have selected the Easy difficulty level.");
                play_game(10);
            }
            2 => {
                println!("Great! You have selected the Medium difficulty level.");
                play_game(5);
            }
            3 => {
                println!("Great! You have selected the Hard difficulty level.");
                play_game(3);
            }
            4 => play_again = false,
            _ => (),
        };
    }
}

fn play_game(chance: u8) {
    let mut rng = rand::thread_rng();
    let number: u8 = rng.gen_range(1..=100);
    let mut attempt: u8 = 0;

    println!("Let's start the game!\n");

    while attempt < chance {
        let mut input = String::new();

        print!("Enter your guess: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let guess: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                clearscreen::clear().expect("failed to clear screen");
                println!("Invalid input. Please enter a number between 1 and 100.");
                attempt += 1;
                continue;
            }
        };

        if guess < 1 || guess > 100 {
            clearscreen::clear().expect("failed to clear screen");
            println!("Invalid guess. Try again");
            attempt += 1;
            continue;
        }

        attempt += 1;

        if guess < number {
            println!("Incorrect! The number is greater than {guess}.");
            println!();
            continue;
        } else if guess > number {
            println!("Incorrect! The number is less than {guess}.");
            println!();
            continue;
        }

        println!("Congratulations! You guessed the correct number in {attempt} attempts.");
        return;
    }

    println!("Failed! Try again.");
    println!();
}

fn start_game() -> u8 {
    println!("Welcome to the Number Guessing Game!");
    println!("I'm thinking of a number between 1 and 100.");
    println!("You have 5 chances to guess the correct number.\n");

    println!("Please select the difficulty level:");
    println!("1. Easy (10 chances)");
    println!("2. Medium (5 chances)");
    println!("3. Hard (3 chances)");
    println!("4. Exit");

    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut choice: u8 = input.trim().parse().unwrap();

    while choice < 1 || choice > 4 {
        clearscreen::clear().expect("failed to clear screen");
        println!("Invalid choice. Try again");

        println!("Please select the difficulty level:");
        println!("1. Easy (10 chances)");
        println!("2. Medium (5 chances)");
        println!("3. Hard (3 chances)");
        println!("4. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        choice = input.trim().parse().unwrap();
    }

    choice
}
