use std::io;

fn main() {
    let mut input = String::new(); // Creates a new String
    io::stdin().read_line(&mut input).unwrap();
    println!("Your input is {}", input);
    /*
    What this means is that we borrow (&) the binding input, and
    it must be mutable, since it will change it's value
    */
}
