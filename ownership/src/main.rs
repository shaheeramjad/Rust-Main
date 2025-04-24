// This program demonstrates the concept of ownership in Rust by creating a string variable
// and attempting to access it after its scope has ended. It will result in a compilation error
// because the variable `x` is no longer valid outside its block scope.

// fn main() {
//     let s: &str = "hello";

//     {
//         let x: &str = "world";
//     }
//     println! {"X" , {x}};
//     println! {"s" , {s}};
// }

fn main() {
    let mut s: String = String::from("hello");
    s.push_str(", world");
    println! {"s = {s}"};
}
