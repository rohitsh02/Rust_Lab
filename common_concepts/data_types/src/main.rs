//! Integer Types
fn main() {
let guess: u32 = "42".parse().expect("Not a number!");
}

//! Floating-Point Types
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}

//! Numeric Operations
fn main() {
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
}

//! The Boolean Type
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}

//! The Character Type
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

//! The Tuple Type
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

//! The Array Type
fn main() {
    let a = [1, 2, 3, 4, 5];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
}

//! Array Element Access
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

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

