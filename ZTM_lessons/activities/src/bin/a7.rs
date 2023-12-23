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
enum Colors {
    Black,
    White,
    Red,
    Blue,
    Green,
    Purple,
    Violet
}

fn color_name(color: Colors) {
    match color {
        Colors::Black => println!("My Favorite Color is Black."),
        Colors::White => println!("My Favorite Color is White."),
        Colors::Red => println!("My Favorite Color is Red."),
        Colors::Blue => println!("My Favorite Color is Blue."),
        Colors::Green => println!("My Favorite Color is Green."),
        Colors::Purple => println!("My Favorite Color is Purple."),
        Colors::Violet => println!("My Favorite Color is Violet."),

    }
   
}

fn main() {
    color_name(Colors::White);
}
