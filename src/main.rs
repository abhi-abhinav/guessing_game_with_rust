use std::io;
use std::cmp::Ordering;  //Ordering is an enum
use rand::Rng;

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret_number is: {secret_number}");


     io::stdin()  
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!"); /*trim eliminates
                                                                            * spaces and new line
                                                                            * parse converts string
                                                                            * to a number */

    println!("You guessed: {guess}");

    loop {
        println!("Please input your guess.");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
