# Imports & Namespaces
In many languages it is common to import libraries from external, or standard libraries, in our case we will only consider the standard library until we talk about [Cargo in Section 6](06-cargo.md). So I guess the best way to start is by first figuring out how to import a library, and that is done by the `use` keyword. 

To help with this section, we will create a simple user input program by using the standard library input/output module ,`std::io`. 

```rust
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
```
In this example, we specify that we only want to use the `io` module 