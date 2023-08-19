fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1];
    for i in 0..xs.len() + 1 {
        // OOPS, one element too far
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Out of bound indexing causes runtime error
    //println!("{}", xs[5]);
}
