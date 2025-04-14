fn main() {
    let mut x: i64 = 0;
    let mut i: i64 = 0;
    for i in 0..10000000000 {
        x += 1;
    }
    println!("{}", x);
}
