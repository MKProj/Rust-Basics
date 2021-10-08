# Structs & Enums

Compared to other languages like C++, Rust does not offer Classes, however instead it's concepts are 
found in structs & enums which are used to create custom types in programs. Let's first start of with structs 
which are common to find in C#, Java, C etc. 

## Structs 
There are three type of structs that exist in Rust, we have: 
1. Empty Structs 
2. Tuple Structs 
3. Structs 

An empty struct is literally what you think, it contains no parameters and tend to be used as a placeholder in programs. A struct can be defined as `stuct <name>`, so an empty struct would look like: 

```rust
struct IamEmpty;
```

A tuple struct is a struct that has no parameters, but instead just asks for the data type, this you can see right below: 

```rust
struct tupleStruct(i32, f32, String);
```
As you can see it looks like a tuple, and doesn't set parameters with actual names.  
The last one is the classic C struct where parameters and data types are provided, and is the most common type of struct you'll use or encounter. 

```rust
struct classic{
    a: i32, 
    b: f32, 
    c: String
}

fn main(){
    let c: classic = classic{
        a: 2, 
        b: 7.8, 
        c: "Hello".to_owned()
    }; //To invoke a struct
}
```
Enums are more custom, as it's not necessary to actually have a data type, of course depending on the context of the program. Let's create a color enum: 

```rust
enum Color{
    Red(u8), 
    Green(u8),
    Blue(u8)
}

fn main(){
    let red = Color::Red(255);
    let green = Color::Green(255);
    let blue = Color::Blue(255);
}
```

Of course this is still limiting of what we can do, and that's where implementations come in, these provide functions for our structs or enums. Now let's create a struct for Points and put some functions in it. 

```rust
//To make implementations we need to use the impl keyword
struct Points{
    x: f32,
    y: f32,
}

impl Points{
    fn new (x: f32, y: f32)-> Self {
        //Similar to a constructor 
        Self{
            x,
            y
        }
    }
    fn slope(&self, other: &Points)-> f32{
        (other.y - self.y) / (other.x - self.x)
    }
    fn midpoint(p1: &Points, p2: &Points)-> Self{
        Self{
            x: (p1.x + p2.x) / 2.0, 
            y: (p1.y + p2.y) / 2.0
        }
    }
}

fn main(){
    let p1 = Points::new(7.8, 8.9);
    let p2 = Points::new(9.8,6.7);

    let midpoint = Points::midpoint(&p1,&p2);
    //Any methods returning Self use ::
    //Any methods not returning Self use .

    println!("The midpoint of p1 to p2 is x: {} y: {}", midpoint.x, midpoint.y);
    //The midpoint of p1 to p2 is x: 8.8 y: 7.7999997
    println!("While the slope of p2 to p1 is {}", p1.slope(&p2));
    //While the slope of p2 to p1 is -1.0999999
}
```
<!--
- Empty struct
- Tuple struct
- classic strut

- enums 

- implementations >