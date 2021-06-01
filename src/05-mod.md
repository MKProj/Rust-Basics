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

### Return Type 
When you write a function in Rust, you can either return nothing, in other 
languages that may be called a `void` function, or you can return a data type using `fn foo()->type{}`. For our example we will create a simple add function to show an easy example to return: 

```rust
fn add(a:i32, b:i32)->i32{
    a + b 
}
//Another way of returning is: 
fn add_alt(a:i32, b:i32)->i32{
    a + b
}
fn main(){
    let a = add(2,5);
    let b = add_alt(2,5);

    if a == b{
        println!("Same result");
    }
}
```

## Modules 
To introduce modules we will be writing a program that involves two files, 
we will respectively have `lib.rs` and `main.rs`. We will have two different sections or modules in our library, one being for geometry of 2D shapes and the other for 3D. To define a module, we must use the `mod` keyword.

> To have functions able to use outside of the module or library, make sure
> to use the `pub` keyword to make it public (private by default). 

```rust
//lib.rs
mod 2D{
    pub fn Area(len: f64, width: f64)->f64{
        len*width; 
    }
}
mod 3D{
    pub fn Volume(len: f64, width: f64, height: f64){
        len * width * height;
    }
}
```

Now to import our library, we must first `mod` it then we can `use` the 
library, so let's do that. 

```rust
//main.rs 
mod lib; 
use lib::2D;
use lib::3D;

fn main(){
    /* 
    Let's find the area and volume of 
    a square and cube respectively. 
    */  
    let side = 2.56;
    println!("Area: {}", Area(side, side));
    println!("Volume: {}", Volume(side, side, side));
}
```