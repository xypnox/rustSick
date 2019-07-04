use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // We use the rand crate to generate a keyring and then call a gen_range
    // to get a number in the range [1, 101)
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);

    // loop forever unless break;
    loop {
        println!("Please input your guess. ");
        // declare a new mutable variable named "guess"
        let mut guess = String::new();

        // Read Input from user
        io::stdin()
            // we could have also written std::io::stdin() and not import the
            // std::io in first line
            .read_line(&mut guess)
            .expect("Failed to read line"); // Shows error handling by default

        //  Convert guess to a u32 number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // Err is a catchall to catch anything except a num
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // We match the cmp's return value with the Ordering enum variants
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                // We finally break the loop if guess is correct
                break;
            }
        }
    }
}
