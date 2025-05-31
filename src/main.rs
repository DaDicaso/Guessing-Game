use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("-----Guess The Number------");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess).expect("Failed to read line");
    
    println!("You guessed: {guess}");
    // converting guess(string) to guess(i32 or u32) type by trimming and parsing 
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    println!("The secrete number is:  {secret_number}");
}