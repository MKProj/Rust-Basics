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