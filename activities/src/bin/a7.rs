// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

fn main() {
    enum Color{
        Red,
        Green,
        Blue,
        White,
        Black
    }

    // With match
    print!("Enum: ");
    let color = Color::White;
    match color {
        Color::Red => print!("Red"),
        Color::Green => print!("Green"),
        Color::Blue => print!("Blue"),
        Color::White => print!("White"),
        Color::Black => print!("Black"),
        _ => println!("Unknown")
    }

    println!();

    // With Function
    print!("Function: ");
    fn print_color(color: Color) {
        match color {
            Color::Red => println!("Red"),
            Color::Green => println!("Green"),
            Color::Blue => println!("Blue"),
            Color::White => println!("White"),
            Color::Black => println!("Black"),
            _ => println!("Unknown")
        }
    }
    print_color(color);
}
