# Bindings & Mutability
In Rust, another way of calling a binding is a variable. A binding is used as a placeholder to store memory of a particular value, and in Rust, by default is set to immutable. Before talking more about this, let's first look at how to declare a binding, one with type inference, and the other with an explicit type: 

```rust
let a = 5.0; //Inferred by default as float64 or f64
let b: f64 = 5.0; //Explicitly declared as type f64
```

To delcare a binding, you must use the `let` keyword with a purposeful name 
and an assignment operator, `=`, with an appropriate value with it. Due to Rust 
being a safe memory language, all bindings are immutable by default, and this is different than most languages, however you'll see most of the times mutability isn't 
necessary. But that doesn't mean mutability isn't possible, it is instead initiated with the convenient `mut` keyword. 

```rust
fn main(){
    let mut a = "I am Mutable"; 
    a = "I can change values!!!"; 
    
    let b = "I am immutable"; 
    b = "This will cause an error!";
    //Press play to check the error!
}
```

## Data Types
---
Rust has four main data types, which are the following: 
- Integers (non decimal point numbers) 
  - Signed: `i8, i16, i32, i64, i128, isize`
  - Unsigned: `u8, u16, u32, u64, u128, usize`
- Floating Points (decimal point numbers)
  - `f32 or f64(inferred default)`
- Boolean
  - `bool`, either evaluates to `true or false`
- Characters 
  - `char`, such as `'L'`

Another type that is commonly used but not primitive is Strings
and is denoted by `String`. 