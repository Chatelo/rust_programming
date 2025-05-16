// fn main() {
//     // --------------------------Ownership Rules------------------
//     /*
//     1. Each value in Rust has an owner.
//     2. There can only be one owner at a time.
//     3. When the owner goes out of scope, the value will be dropped.
//      */


//     // s is not valid here, since it's not yet declared
//     let s = "hello"; // s is valid from this point forward
//     // do stuff with s

// }
// this scope is now over, and s is no longer valid


// fn main(){
//     // let s = String::from("hello");
//     let mut s = String::from("hello");
//     s.push_str(", world"); // push_str() appends a literal to a string
//     println!("{s}")
// }


// Variables and Data Interacting with Move


// fn main(){
//     let x = 5;
//     let y = x;
// }

// fn main(){
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("{s1}, world")
// }
// Variables and Data Interacting with Clone
// fn main(){
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
// println!(" s1 = {s1}, s2 = {s2}")
// }

// Stack-Only Data: Copy
// fn main(){
//     let x = 5;
//     let y = x;
//     println!("x = {x}, y = {y}");

// }

// ****************Ownership and Functions**************************
// fn main(){
//     let s = String::from("hello"); // s comes into scope
//     takes_ownership(s); // s's value moves into the function
//     // s is no longer valid here

//     let x = 5; // x comes into scope
//     makes_copy(x); // x would move into the function, but i32 is Copy
    // x is still valid here
       
// } // here x goes out of scope, then s. However, because s's value was moved, nothing special happens.

// fn takes_ownership(some_string: String){ // some_string comes into scope
//     println!("{some_string}")

// }
// Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

// fn makes_copy(some_integer: i32){ // some_integer comes into scope
//     println!("{some_integer}")
// }
// Here, some_integer goes out of scope. Nothing special happens.



//******************************Return Values and Scope*************

// fn main(){
//     let s1 = gives_ownership(); // gives_ownersghip moves its return value into s1
//     let s2 = String::from("hello"); // s2 comes into scope
//     let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which
//                                         // also moves its return value into s3


// }
// // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// // happens. s1 goes out of scope and is dropped.
// fn gives_ownership() -> String {
// // gives_ownership will move its
// // return value into the function
// // that calls it
// let some_string = String::from("yours"); // some_string comes into scope some_string and
// // some_string is returned
// // moves out to the calling
// // function
// }
// // This function takes a String and returns a String.
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope a_string
// // a_string is returned and moves out to the calling function
// }

// fn main() {
// let s1 = String::from("hello");
// // let (s2, len) = calculate_length(s1);
// let result:(String, usize) = calculate_length(s1);
// println!("The length of {} is {}.", result.0, result.1);
// }

// fn calculate_length(s: String) -> (String, usize) {
// let length = s.len(); // len() returns the length of a String
// (s, length)
// }


// ***********References and Borrowing****************
// fn main(){
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1); // pass a reference
//     println!("The lenghth of '{s1}' is {len}")
  
// }

// fn calculate_length(s:&String) -> usize { // s is a reference to a String
//     s.len()
// }
// Here, s goes out of scope. But because it does not have ownership of what
// it refers to, the String is not dropped.

// Cannot mutate a borrowed value
// fn main() {
//     let s = String::from("hello");
//     change(&s);
// }

// fn change(some_string: &String){
//     some_string.push_str(", world");
// }

// ****************Mutable References**************

// fn main() {
// let mut s = String::from("hello");
// change(&mut s);

// }
// fn change(some_string: &mut String) {
// some_string.push_str(", world");


// }

// if you have a mutable reference to a value, you can have no other references to that value

// let mut s = String::from("hello");
// let r1 = &mut s;
// let r2 = &mut s;
// println!("{r1}, {r2}");

// // ***************Dangling references************
// fn main() {
// let reference_to_nothing = dangle();
// }
// fn dangle() -> &String {
// let s = String::from("hello");
// &s
// }

// *******************The Rules of References*******************
// Let’s recap what we’ve discussed about references:
// At any given time, you can have either one mutable reference or
// any number of immutable references.
// References must always be valid.

