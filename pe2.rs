struct Fibonacci {
    a: u32,
    b: u32
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let r = self.a + self.b;
        self.a = self.b;
        self.b = r;
        return Some(r);
    }
}

fn main() {
    let fib = Fibonacci { a: 1, b: 1 };
    let r = fib.take_while(|&x| x < 4000000).filter(|&x| x % 2 == 0).fold(0, |i, x| i + x);
    println!("{}", r);
}
