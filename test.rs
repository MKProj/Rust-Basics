use std::io::stdin;
use mkproj_lib::uniconv::temperature;

fn main() {
    let mut input = String::new(); //gets input
    stdin().read_line(&mut input).unwrap();

    let temp: f32 = input.trim().parse().unwrap();


}
