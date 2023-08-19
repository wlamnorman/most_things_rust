fn max<'a>(x: &'a f64, y: &'a f64) -> &'a f64 {
    if x > y {
        x
    } else {
        y
    }
}

fn simple_function() {
    let a = String::from("Keke.");
    println!("{}", a);
  }


fn main() {
    let x: f64 = 64.0;
    let y: f64 = 323.01;
    println!("max of x and y is {}", max(&x,&y));

    simple_function();
}
