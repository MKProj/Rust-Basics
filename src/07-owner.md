# Ownership Basics

Rust follows a memory/thread safe, zero abstraction model and due to that a Garbage Collector won't comply. A GC automatically 
handles the drops and move between data in the program, and due to that it costs some performance for that memeory management. 

So what do we do then? Well Rust uses ownerships rules to comply with Rust's Borrow Checker that ensures memory safety, however
in this book we won't go into much detail of Rust's handling with stack and heap allocation, but we will show enough to avoid 
fighting the borrow checker. 

### Ownership Rules
- Each value in Rust has a variable that's called it's __owner__
- There can only be *one* owner at a time
- When the owner goes out of scope, the value will be *dropped*

We can see a simple example using closures: 
```rust
{ //my_var isn't valid, isn't declared yet'
    let my_var = "My Variable exists here!"; //valid from this point forward
    //Do stuff with my_var
} //The scope is over, so my_var is dropped
```

Now we introduce the concept of borrowing, and this is done by using the `&` operator. Borrowing allows other 
variables to use a variables data, and we have two ways of borrowing, we have `&` (immutable borrow) and `&mut` 
(mutable borrow) where mutable borrow allows for us to manipulate the data. 

```rust
// This program borrows a vec and pushes 2 into it 
fn push_two(v: &mut Vec<i32>){
    v.push(2);
}
fn main(){
    let mut v = vec![1,6,7,8];
    push_two(&mut v);
    println!("{:?}", v)
}
//Result: [1, 6, 7, 8, 2]
```


### String and strs 
If you havent noticed, but Rust has two different type of string types, `String` and `&str`, and this 
can be explained due to ownership. A `String` variable is an owned value, and this means if you move the value from
one variable to another, the new variable now owns the value and the other variable is dropped. Now the other string type 
`&str` is the reference to a string, or a borrowed version. So if you move one value to the other, both are still valid, 
and these both have it's use cases and it's important to note that in functions it is common to use `&str` instead of `String` (due to borrowing the parameter). 

This guide doesn't go into the finer details of the ownership rules in Rust and it is highly recommend to read the documentation provided by the Rust Foundation [here for more details.](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)