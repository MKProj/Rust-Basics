use rand::Rng;// For rand numbers
use std::cmp::{Ord, Ordering};// For Ordering 
fn main() {
    let rng = rand::thread_rng().gen_range(1..10);//Our random number generator

    let mut s = String::new();//Use to get user input
    println!("Enter your guess: ");// Ask for input
    std::io::stdin().read_line(&mut s).unwrap();

    let guess: i32 = s.trim().parse().unwrap();

    //Now we match our guess to compare with our random number
    match guess.cmp(&rng) {
        Ordering::Greater => println!("Too high!"),
        Ordering::Less => println!("Too low!"),
        Ordering::Equal => println!("You're right!"),
    }
}
