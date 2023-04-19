// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn main() {
    let int_value: i32 = 10;
    match int_value {
        6..=i32::MAX => println!("Greater than 5."),
        i32::MIN..=4 => println!("Less than 5."),
        _ => println!("It's 5."),
    }
}
