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

enum Flavours {
    ColaCola,
    Fanta,
    Pepsi,
}

struct Information {
    flavor: Flavours,
    quantity: f64,
}

fn print_information(information: Information){
    match information.flavor{
        Flavours::ColaCola => println!("Cola Cola"),
        Flavours::Fanta => println!("Fanta"),
        Flavours::Pepsi => println!("Pepsi"),
        
    }

    println!("Quantity: {:?}", information.quantity);
}
 
fn main() {
    let drink_today = Information{
        flavor: Flavours::ColaCola,
        quantity: 250.00,
    } ;
    
    print_information(drink_today);
    let drink_yesterday = Information{
        flavor: Flavours::Fanta,
        quantity: 200.00,
    } ;
    
    print_information(drink_yesterday);
    let drink_tomorrow = Information{
        flavor: Flavours::Pepsi,
        quantity: 500.00,
    } ;
    
    print_information(drink_tomorrow);
}
