// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {
    let value = 5;
    if value < 5 {
        println!("Value is less than 5");
    }else if value > 5 {
        println!("Value is greater than 5");
    }else{
        println!("Value is equal to 5");
    }

    // With Match
    let int_value: i32 = 10;
    match int_value {
        6..=i32::MAX => println!("Greater than 5."),
        i32::MIN..=4 => println!("Less than 5."),
        _ => println!("It's 5."),
    }
}
