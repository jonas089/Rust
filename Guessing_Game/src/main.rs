/* "prelude" from the standard library
the "prelude" is the list of things that Rust automatically imports
into every Rust program. The std::io library contains useful things
such as handling of user input.
*/
use std::io;

fn main(){
    println!("Guess the number!");
    println!("Please input your guess.");
    /* a variable is created to store the user input.
    the "let" statement declares the variable.
    A variable is immutable by default. To make a variable
    mutable, "mut" is added before the variable name.
    */
    let mut guess = String::new(); // new() is an associated function of the String type.
    io::stdin()
        .read_line(&mut guess) // read line of input and store input in guess variable.
        /* the "&" indicates that this argument is a reference. References enable multiple
        parts of the code to access a piece of data without needing to copy the data into
        memory multiple times.
        */
        .expect("Failed to read line"); // an exception. Not handling a possible error will produce a warning.
    println!("You guessed: {}", guess); //{} functions as a placeholder for the guess variable.
}
