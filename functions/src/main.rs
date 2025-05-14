// fn main() {
//     println!("Hello, world!");
//     another_function();
// }
// fn another_function() {
// println!("Another function.");
// }
// Functions can be defined in any order
// Functions are defined with the `fn` keyword
// Functions can take parameters
// Functions can return values
// Function names are in snake_case

// fn main() {
//     another_function(5);
// }
// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// // Parameters must have a type
// fn main() {
//     print_labeled_measurement(5, 'h');
// }
// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// ****Expression vs Statement****

// fn main() {
//     // Statements do not return a value
// let y = 6;
// }

// fn main() {
//     // 
// let x = (let y = 6);
// }

// Functions with return values
// fn five() -> i32 {
//     5
// }
// fn main() {
//     let x = five();
//     println!("The value of x is: {x}");
// }

// fn main() {
//     let x = plus_one(6);
//     println!("The value of x is: {x}");

// }

// fn plus_one(x: i32) -> i32{
//     x + 1
// }

// control flow
fn main() {
    let number = 3;
if number < 5 {
    print!("condition was true");
} else {
    print!("condition was false");
}
}
