# Cargo
Cargo is Rust's package manager utility and is the recommended way to build a Rust program as it allows the ability to import crates, and publish your own. It is very convenient to the user, and offers two different type of projects to be built: 
- bin : A binary project is a project to be executable (default)
- lib: A library is a way to store all your functions, modules, etc. seperately
A crate in Rust is a cargo project published in Rust's `(http://crates.io)[crates.io]`.  
 To create a new cargo project, we use the `cargo new` option, and provided a name will create a project for us with the following structure: 
 ```bash
$ cargo new myproject # Create binary crate myproejct
$ cd myproject
$ ls
Cargo.toml src
 ```

 The `src` directory contains all of our source code, and inside the folder contains a `main.rs` file for us to do our work. 
 The `Cargo.toml` file contains all of the metadata for out project such as name, author, version number, dependencies, etc. 
 For more information about what can be done in Cargo, you can visit (https://doc.rust-lang.org/cargo/index.html)[here]. 

 For our example, we will make use of the `rand` crate to create randomized numbers for a guessing game. 

 ```bash
 $ cargo new guess_game # create a new guess_game
 $ cargo search rand # let's find the version number of the crate
 rand = "0.8.4"      # Random number generators and other randomness functionality.
 ```

 Let's first edit our `Cargo.toml` file and add the `rand` crate as a dependency.
 ```toml
 #Cargo.toml
[package]
name = "myproject"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.4"
 ```
Now that we have rand as a dependency,we are free to use it in our project. 
Now we can edit our `src/main.rs` file and begin our guessing game that does the following: 
 - The user only has one attempt to try
 - We will tell them if they are too low or too high
 - They will have to guess from 1 to 10

 ```rust
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
 ```

 You can try yourself running `cargo run` and see if you guess right. You may also build 
 the project by running, `cargo build`, and the executable will be found in `target/debug/<name>`. 