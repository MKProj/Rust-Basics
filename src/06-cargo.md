# Cargo
Cargo is Rust's package manager utility and is the recommended way to build a Rust program as it allows the ability to import crates, and publish your own. It is very convenient to the user, and offers two different type of projects to be built: 
- bin : A binary project is a project to be executable
- lib: A library is a way to store all your functions, modules, etc. seperately

Let's create a Unit Converter program that for our need will only convert Kelvin or Fahrenheit units to Celsius. 

```bash
# First create our binary project 
$ cargo new unit_converter # default is bin
     Created binary (application) `unit_converter` package

# Go to the project and check it's content
$ cd unit_converter && ls
Cargo.toml  src

# Let's run the program
$ cargo run
   Compiling unit_converter v0.1.0 (/home/user/unit_converter)
    Finished dev [unoptimized + debuginfo] target(s) in 2.12s
     Running `target/debug/unit_converter`
Hello, world!
```
Now if you noticed, you'll see that program ran from a `target` directory, and when we first did `ls` we didn't see it. Well that's because it doesn't appear until you first run the program. 

Now we need to import a library, for our example we will be using the `uniconv` module in the `mkproj-lib` crate. You can either go to [crates.io](http://crates.io) and search it up, or you can use `cargo search mkproj-lib`. Let's try `cargo search`, and if you want to look more into what `mkproj-lib` contains, visit it's [documentation](http://docs.rs/mkproj_lib)!

```bash
$ cargo search mkproj-lib
mkproj-lib = "0.2.10"    # Modules for MKProject Programs
```
 To first use the crate, we must have it as a dependecies, and that is under `Cargo.toml`.   
 The `Cargo.toml` is used to contain the metadata of the project, such as name, version, dependencies, etc. 


 ```toml
[package]
name = "unit_converter"
version = "0.1.0"
authors = ["Name <email>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/refere>

[dependencies]
mkproj-lib = "0.2.10" # We have our crate here 
 ```

 Now let's go to our `main.rs` file that will be under the `src` directory, and we will get user input for temperature and whether it's 
 Fahrenheit or Kelvin. 
 