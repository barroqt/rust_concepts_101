// (1 °F − 32) × 5/9 
use std::io;

fn main() {
    let mut fahrenheit = String::new();

    println!("input Fahrenheit:");
    io::stdin().read_line(&mut fahrenheit).expect("Could not read line");

    let fahrenheit: f64 = fahrenheit.trim().parse().expect("Must be a number");

    let celsius: f64 = (fahrenheit - 32.0) * (5.0/9.0);
    println!("Degree celsius: {}", celsius);
}
