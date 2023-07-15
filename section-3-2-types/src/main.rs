use std::io;

fn main() {
    let guess = "42".parse::<u8>().expect("Not a number!");

    println!("guess {guess}");

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("x {x} y {y}");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("sum {sum}");
    println!("difference {difference}");
    println!("product {product}");
    println!("quotient {quotient}");
    println!("truncated {truncated}");
    println!("remainder {remainder}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("tuple {} {} {}", tup.0, tup.1, tup.2);

    let a = [1, 2, 3, 4, 5];

    println!("array {:?}", a);

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
