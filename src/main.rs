use std::io;
use rand::Rng;

fn main() {
    println!("-----Guess The Number------");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Please input your guess: ");

    //let apple = "Hello From apple"; // immutable
    let mut guess = String::new(); // mutable

    io::stdin()
        .read_line(&mut guess).expect("Failed to read line");
    
    println!("Your guess and update {}", guess);


}