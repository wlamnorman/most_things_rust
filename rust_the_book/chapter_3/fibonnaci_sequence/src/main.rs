fn main() {
    let n: i32 = 5;
    let mut a: i32 = 1;
    let mut b: i32 = 1;
    for _ in 0..n {
        println!("{}", a);
        let next = a + b;
        a = b;
        b = next;
    }
}
