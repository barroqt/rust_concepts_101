fn main() {
    let mut n: u32 = 1;
    let mut prev: u32 = 0;
    let mut sum: u32;

    for i in 1..15 {
        println!("iteration: {}", i);

        sum = prev + n;
        prev = n;
        n = sum;

        println!("Fibo: {}", n);
    }
}
