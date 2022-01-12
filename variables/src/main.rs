use std::io;

fn main() {
    let x = 5;
   
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in scope is {}", x); 
    }
    println!("The value of x is {}", x); 

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is {}, y is {}, z is {}", x, y, z); 

    // Accessing elements of tuple x
    // let five_hundred = x.0;

    // let six_point_four = x.1;

    // let one = x.2;

    let a = [1, 2, 3, 4, 5]; // data allocated on the stack rather than the heap
    // Arrays are preferable only when it's not gonna change, in size or values. Ex: a calendar with the fixed months of the year
    // Use of vectors instead 
    
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // = let a = [3, 3, 3, 3, 3];

    // Showing that rusts does not let you access invalid memory by checking the array's length

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index:");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read the line");

    let index: usize = index.trim().parse().expect("Index entered is not a number");

    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);
}
