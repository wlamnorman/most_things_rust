#[derive(Debug)]
enum Shape {
    Circle(f64) // radius
    ,Rectangle(f64, f64) // width, height
    ,Triangle(f64, f64, f64) // a, b, c where b is the 
}

impl Shape {
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(r) => 2.0*r*std::f64::consts::PI
            ,Shape::Rectangle(w,h) => 2.0*w+2.0*h
            ,Shape::Triangle(a,b,c) => a+b+c
        }
    }
}


enum Location {
    Unknown
    ,Anonymous
    ,Known(f64, f64)
}

impl Location {
    fn display(&self) -> () {
        match self {
            Location::Unknown => println!("The location is unknown.")
            ,Location::Anonymous => println!("The location is anonymous.")
            ,Location::Known(x,y) => println!("The location is ({}, {})", x, y)
        }
    }
}

fn main() {
    // let my_shape = Shape::Rectangle(2.0,3.0);
    // let perimeter = my_shape.perimeter();
    // println!("my_shape is {:?} with perimeter {}", my_shape, perimeter);

    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608, -80.604);
    address.display();
}
