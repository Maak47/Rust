// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Tickets {
    BackStage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    let person1 = "Jack".to_owned();
    let person2 = "Mary".to_owned();
    let person3 = "Micky".to_owned();
    let event = vec![
        Tickets::BackStage(50.0, person1),
        Tickets::Vip(30.0, person3),
        Tickets::Standard(15.0),
    ];
    for ticket in event {
        match ticket {
            Tickets::BackStage(price, holder) => {
                println!("BackStage ticket HOLDER: {:?}, price: {:?}", holder, price)
            }
            Tickets::Vip(price, holder) => {
                println!("Vip ticket HOLDER: {:?}, price: {:?}", holder, price)
            }
            Tickets::Standard(price) => {
                println!("Standard ticket Price: {:?} ", price)
            }
        }
    }
}
