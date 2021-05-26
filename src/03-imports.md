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
In this example, we specify that we only want to use the `io` module in the 
`std` library. However let's say we don't know what we really want from the `std` 
library, and instead of guessing, we could just import everything by using `*` after 
`std::`. 

Consider the example below: 
```rust
use std::*;

fn main() {
    //We will be copying one file to another
    let mut a = fs::File::create("file.txt").expect("file not created");

    //The file we will be copying from
    let mut b = fs::File::open("copy.txt").expect("file not found");
    //Using io to copy b to a
    io::copy(&mut b, &mut a).expect("file cannot be copied");
}
```

We get the following result: 
```bash
$ rustc ex.rs
$ ./ex

#copy.txt
This file has words!!!

#file.txt
This file has words!!!
```

As you can see we could access anything from `std` using `*`, however it isn't the
most effective thing to do, in this example we only needed `io` and `fs` modules, so 
let's just import those. 

```rust
use std::{fs, io};

fn main() {
    //We will be copying one file to another
    let mut a = fs::File::create("file.txt").expect("file not created");

    //The file we will be copying from
    let mut b = fs::File::open("copy.txt").expect("file not found");
    //Using io to copy b to a
    io::copy(&mut b, &mut a).expect("file cannot be copied");
}
```
As you can see, if you know what you want to import, and need multiple modules, then
surround them with `{ }`. 

> Keep in mind, when it comes to namespaces, whatever the module you import is the name
> you must start with. Example `std::io` you use `io::`, but if you did `std::fs::File`, you must start with `File::`