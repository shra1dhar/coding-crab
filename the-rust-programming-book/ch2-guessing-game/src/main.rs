use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess");

    let secret_number = rand::thread_rng().gen_range(0, 100);
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: i32 = guess.trim().parse().expect("Failed to parse the input");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Number is small"),
        Ordering::Greater => println!("Number is larger"),
        Ordering::Equal => println!("Number is equal, you win !!"),
    }
}
