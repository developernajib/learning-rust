// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn name(fname: &str, lname: &str) -> String {
    fname.to_string() + " " + lname
}

fn main() {
    println!("{}", name("John", "Doe"));
}