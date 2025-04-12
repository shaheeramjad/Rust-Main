// This program demonstrates the use of mutable and immutable variables in Rust.
// It shows how to declare variables, assign values, and print them to the console.
// It also illustrates the concept of shadowing, where a new variable with the same name

fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    // constant();
    //shadowing();
    //calculation();
    compound();
}

//Constants
// fn constant() {
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("The value of three hours in seconds is: {THREE_HOURS_IN_SECONDS}");
// }

//Shadowing

// fn shadowing() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

//Calculation aka dataTypes
// fn calculation() {
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

//Compound Types
fn compound() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
