/* "prelude" from the standard library
the "prelude" is the list of things that Rust automatically imports
into every Rust program. The std::io library contains useful things
such as handling of user input.
*/
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main(){
    println!("Guess the number!");
    loop{
        println!("Please input your guess.");
        /* a variable is created to store the user input.
        the "let" statement declares the variable.
        A variable is immutable by default. To make a variable
        mutable, "mut" is added before the variable name.
        */
        let mut guess = String::new(); // new() is an associated function of the String type.
        let secret_number = rand::thread_rng().gen_range(1..101); // unless specified defaults to a 32-bit number
        io::stdin()
            .read_line(&mut guess) // read line of input and store input in guess variable.
            /* the "&" indicates that this argument is a reference. References enable multiple
            parts of the code to access a piece of data without needing to copy the data into
            memory multiple times.
            */
            .expect("Failed to read line"); // Throw an Error.
        //let guess: u32 = guess.trim().parse().expect("Please type in a number!");
        /* shadow the variable to change type
        the parse method on strings will parse them into some type of number. The type bust be specified. e.g. u32: 32-bit number.
        */
        // Handling the Invalid input as an exception:
        /* instead of using the expect expression, we can use a match expression,
        to handle the error instead of crashing.
        */
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {}", guess); //{} functions as a placeholder for the guess variable.
        println!("Secred number: {}", secret_number);
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!");
                break
            }
        }
    }
}
