use rand::Rng; // rand crate is used to generate random numbers
use std::cmp::Ordering; // cmp crate is used to compare two numbers
use std::io; // io crate is used to read input from the user

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101); // rand::thread_rng() generates a random number and gen_range generates a random number between 1 and 100
    // println!("The secret number is {}", secret_number);                                                          

    loop { // loop is used to run the program continuously
        
        println!("Please input your guess.");

        let mut guess = String::new(); // mutable string

        io::stdin()
            .read_line(&mut guess) // read_line() is used to read input from the user and store it in the guess variable
            .expect("Failed to read line"); // expect() is used to handle errors

        let guess: u32 = match guess.trim().parse() {
            // .trim is used to remove whitespace from the input and .parse() is used to convert the input to a number
            Ok(num) => num, // Ok() is used to handle successful conversions
            Err(_) => {
                // Err() is used to handle unsuccessful conversions and _ is used to ignore the error
                println!(" Error: Please imput a number.");
                continue; // continue is used to start the loop over again
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) { // & is used to reference the value of the variable
            // cmp() is used to compare two numbers
            Ordering::Less => println!("Too small"), // Ordering::Less is used to handle numbers that are less than the secret number
            Ordering::Greater => println!("Too big"), // Ordering::Greater is used to handle numbers that are greater than the secret number
            Ordering::Equal => {
                // Ordering::Equal is used to handle numbers that are equal to the secret number
                println!("You win");
                break; // break is used to break out of the loop
            }
        }
    }
}
