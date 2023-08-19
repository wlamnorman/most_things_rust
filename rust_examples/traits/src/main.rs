struct Satellite {
    name: String,
    velocity: f64 // km/s 
}

impl ...



fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };

    println!("Hubble is {}", hubble.velocity);
}
