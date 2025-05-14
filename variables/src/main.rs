// Variables are immutable by default - you can use the `mut` keyword to make them mutable
// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// Variables shadowing - you can use the same variable name to create a new variable
// fn main(){
//     let x = 5;
//     let x = x + 1; // This shadows the previous value of x
//     {
//         let x = x * 2; // This shadows the previous value of x
//         println!("The value of x in the inner scope is: {}", x);
//     }
//     println!("The value of x in the outer scope is: {}", x);
// }

// Shadowing allows you to change the type of a variable
// fn main() {
//     let spaces: &'static str = "   ";
//     let spaces: usize = spaces.len(); // This shadows the previous value of spaces 
//     println!("The length of spaces is: {}", spaces);
// }
// So this saves us from having to create a new variable with a different name


// Floating point numbers
// fn main() {
//     let x = 2.0; 
//     let y: f32 = 3.0;
//     let sum = x + y as f64; // You can cast a f32 to a f64
//     println!("The sum of x and y is: {}", sum);
// }

// Numeric operations
// fn main() {
//     // addition
//     let sum = 5 + 10;
//     // subtraction
//     let difference = 95.5 - 4.3;
//     // multiplication
//     let product = 4 * 30;
//     // division
//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3; // Results in -1
//     // remainder
//     let remainder = 43 % 5;
// }

// Boolean

// fn main() {
// let t = true;
// let f: bool = false; // with explicit type annotation
// println!("The value of t is: {}", t);
// println!("The value of f is: {}", f);
// }

// Character type
// fn main() {
// let c = 'z';
// let z: char = 'ℤ'; // with explicit type annotation
// let heart_eyed_cat = '😻';
// println!("The value of c is: {}", c);
// println!("The value of z is: {}", z);
// println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);
// // The char type is four bytes in size and represents a Unicode scalar value
// // It can represent any character in the Unicode standard, including emojis
// }


// *********Compound types***********

// Tuples
// fn main() {
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup; // Destructuring a tuple
    // println!("The value of x is: {}", x);
    // println!("The value of y is: {}", y);
    // println!("The value of z is: {}", z);
    // let five_hundred = tup.0; // Accessing a tuple element by index
    // let six_point_four = tup.1; // Accessing a tuple element by index
    // let one = tup.2; // Accessing a tuple element by index
    // println!("The value of five_hundred is: {}", five_hundred);
    // println!("The value of six_point_four is: {}", six_point_four);
    // println!("The value of one is: {}", one);
    // Tuples can be used to return multiple values from a function
// }

// Arrays
// fn main(){
//     let a = [1, 2, 3, 4, 5];
//     let first = a[0]; // Accessing an array element by index
//     let second = a[1]; // Accessing an array element by index
//     println!("The value of first is: {}", first);
//     println!("The value of second is: {}", second);
// }
// array must be of the same type
//  array is fixed in size
//  arrays are stack allocated
//  arrays are useful when you know the size of the array at compile time
//  arrays are useful when you want to store a fixed number of elements
//  arrays are useful when you want to store a collection of elements of the same type


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
println!(
"The value of the element at index {index} is: {element}"
);
}