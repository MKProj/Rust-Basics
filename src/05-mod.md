# Modules & Functions

## Function Syntax 
A function in Rust is defined with the `fn` keyword and is a block of code 
that uses parameters to pass arguments and returns a value or values. To create a 
function first use the `fn` keyword along with a name, then put brackets`()` that will 
contain the parameters, and then `{}` for your block of code.

To make this a lot simpler, observe a simple `hello_world()` function that asks 
for a name and prints `"Hello World Name!"`. To do this we will also require a 
parameter name that's a `String`. 

```rust
//Let's define our hello_world function
fn hello_world(name: &String){
// Since we don't want to entirely use the variable 
// We will borrow it using &
    println!("Hello World {}!", name);
}

fn main(){
    let mut name: String = String::from("Bob");

    //To call a function simply use it's name
    hello_world(&name);
}
```