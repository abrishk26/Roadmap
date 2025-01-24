use std::io::{self, Write};

fn main() {
    println!("Welcome to the Number Guessing Game!");
    println!("I'm thinking of a number between 1 and 100.");
    println!("You have 5 chances to guess the correct number.\n");

    println!("Please select the difficulty level:");
    println!("1. Easy (10 chances)");
    println!("2. Medium (5 chances)");
    println!("3. Hard (3 chances)");

    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut choice: u8 = input.trim().parse().unwrap();

    while choice < 1 || choice > 3 {
        clearscreen::clear().expect("failed to clear screen");
        println!("Invalid choice. Try again");

        println!("Please select the difficulty level:");
        println!("1. Easy (10 chances)");
        println!("2. Medium (5 chances)");
        println!("3. Hard (3 chances)");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        choice = input.trim().parse().unwrap();
    }

    match choice {
        1 => println!("Great! You have selected the Easy difficulty level."),
        2 => println!("Great! You have selected the Medium difficulty level."),
        3 => println!("Great! You have selected the Hard difficulty level."),
        _ => (),
    };

    println!("Let's start the game!\n");

    input.clear();
    print!("Enter your guess: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let mut guess: u8 = input.trim().parse().unwrap();

    while guess < 1 || guess > 100 {
        clearscreen::clear().expect("failed to clear screen");
        println!("Invalid guess. Try again");
        
        io::stdout().flush().unwrap();
        
        input.clear();
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        guess = input.trim().parse().unwrap();
    }

    println!("You have guessed {guess}");
}
