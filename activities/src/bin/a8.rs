// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor


fn main() {
    enum Flavor {
        Orange,
        Mango,
        AppleGrape,
    }
    
    struct Drink {
        flavor: Flavor,
        ounces: f32,
    }
    
    fn print_drink_info(drink: Drink) {
        match drink.flavor {
            Flavor::Orange => println!("Orange - {:?}", drink.ounces),
            Flavor::Mango => println!("Mango - {:?}", drink.ounces),
            Flavor::AppleGrape => println!("Apple Grape - {:?}", drink.ounces),
        }
    }

    let orange_drink = Drink { flavor: Flavor::Orange, ounces: 16.5 };
    let mango_drink = Drink { flavor: Flavor::Mango, ounces: 8.0 };
    let apple_grape_drink = Drink { flavor: Flavor::AppleGrape, ounces: 12.0 };

    print_drink_info(orange_drink);
    print_drink_info(mango_drink);
    print_drink_info(apple_grape_drink);
}
