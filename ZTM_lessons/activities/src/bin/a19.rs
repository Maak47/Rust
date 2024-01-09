// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut furnitures = HashMap::new();
    furnitures.insert("Chairs", 5);
    furnitures.insert("Beds", 3);
    furnitures.insert("Tables", 2);
    furnitures.insert("Couches", 0);
    furnitures.insert("Cabinets", 4);

    let mut total_pieces = 0;

    for (furniture, pieces) in furnitures.iter() {
        // MY SOLUTION
        //     total_pieces += pieces;
        //     match pieces {
        //         0 => println!("name: {:?}, out of stock", furniture),
        //         _ => println!("name: {:?}, number: {:?}", furniture, pieces),
        //     }
        // }

        // TUTOR SOLUTION
        total_stock = total_stock + qty;
        let stock_count = if qty == &0 {
            // put a check here
            "out of stock".to_owned() //value that gets alloted to the variable
        } else {
            format!("{:?}", qty) //format macro works like println! but instead of displaying it, it saves the string on a variable
        };
        println!("item={:?}, stock={:?}", item, stock_count);
    }

    println!("total pieces: {:?}", total_pieces);
}
