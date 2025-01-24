# Number Guessing Game

Sample solution for the [number-guessing-game](https://roadmap.sh/projects/number-guessing-game) challenge from [roadmap.sh](https://roadmap.sh).

## Description

The **Number Guessing Game** is a simple console-based game written in Rust where the player tries to guess a randomly generated number within a given range. The game features multiple difficulty levels, each offering a different number of attempts. Players can continue playing until they choose to exit.

## Features

* **Difficulty Levels** :
* **Easy** : 10 chances to guess the number.
* **Medium** : 5 chances to guess the number.
* **Hard** : 3 chances to guess the number.
* Random number generation between 1 and 100.
* Input validation to ensure guesses are within the valid range.
* Feedback on whether the guessed number is too high or too low.
* Clear and user-friendly console output.

## How to Run

### Prerequisites

* Rust installed on your system. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

### Steps

1. Clone the repository or copy the program's source code to a `.rs` file (e.g., `number_guessing_game.rs`).
2. Open a terminal and navigate to the directory containing the file.
3. Run the program using the following commands:
   ```sh
   rustc number_guessing_game.rs
   ./number_guessing_game
   ```

## How to Play

1. Run the program.
2. Select a difficulty level by entering the corresponding number:
   * `1`: Easy
   * `2`: Medium
   * `3`: Hard
   * `4`: Exit the game
3. Enter your guesses as prompted. The program will provide feedback:
   * If your guess is too high, it will say the number is smaller.
   * If your guess is too low, it will say the number is greater.
4. Try to guess the number within the allotted chances.
5. You can play again by selecting a difficulty level or exit by choosing `4`.

## Example Gameplay

```
Welcome to the Number Guessing Game!
I'm thinking of a number between 1 and 100.
You have 5 chances to guess the correct number.

Please select the difficulty level:
1. Easy (10 chances)
2. Medium (5 chances)
3. Hard (3 chances)
4. Exit

Enter your choice: 2

Great! You have selected the Medium difficulty level.
Let's start the game!

Enter your guess: 50
Incorrect! The number is greater than 50.

Enter your guess: 75
Incorrect! The number is less than 75.

Enter your guess: 60
Congratulations! You guessed the correct number in 3 attempts.
```

## Dependencies

* [rand](https://crates.io/crates/rand): Used for generating random numbers.
* [clearscreen](https://crates.io/crates/clearscreen): Used for clearing the console screen.

To include these dependencies, add the following lines to your `Cargo.toml` file:

```toml
[dependencies]
rand = "0.8"
clearscreen = "2.0"
```

## Enjoy the game! Happy guessing!
