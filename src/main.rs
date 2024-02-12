/*-----------------------------------------------------*/
/* Guessing number game using Rust                     */
/*-----------------------------------------------------*/

use std::io;
use rand::prelude::*;

fn main() {
    // Welcome message
    println!("-----------------------------------------------------");
    println!("Welcome to this Guess The Number Game made with Rust!");
    println!("-----------------------------------------------------");
    println!("");

    // Getting from the user the maxiaml range
    println!("Please Type the maximal range (1 to 127):");
    let mut user_range = String::new();
    io::stdin().read_line(&mut user_range).unwrap();
    let maximal_range = match user_range.trim().parse::<u8>() {
        Ok(num) if num > 0 && num <= 127 => num,
        _ => {
            println!("Please type a valid number (1 to 127)");
            return;
        }
    };

    // Getting the randomly generated number
    let random_number = generate_random_number(maximal_range);
    let mut guess_input = String::new();
    let mut lives: u8 = 10;

    // The main loop
    loop {
        println!("Take a guess (remaining lives: {})", lives);

        // Clear the input buffer
        guess_input.clear();

        // Getting the user guess
        io::stdin().read_line(&mut guess_input).unwrap();
        let guess = match guess_input.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };
        if guess > maximal_range {
            println!("Outside the range, remember the maximum number is {}", maximal_range);
        }
        if guess == random_number {
            println!("Congratulations! You won!");
            break;
        }
        println!("Incorrect guess!");
        lives -= 1;
        if lives == 0 {
            println!("Sorry, you lost. The correct number was: {}", random_number);
            break;
        }
    }
}

// Function for generating random numbers within the range of [0,max[
fn generate_random_number(max: u8) -> u8 {
    let mut rng = thread_rng();
    let x: u8 = rng.gen_range(0..max);
    x
}
