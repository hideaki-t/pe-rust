fn main() {
    // sum is an unsable api...
    // let r = (1..10).filter(|i| i % 3 == 0 || i % 5 == 0).sum::<i32>();
    let r = (1..1000).filter(|i| i % 3 == 0 || i % 5 == 0).fold(0, |i, x| i + x);
    println!("{}", r);
}
