// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    quantity: i32,
    id_number: i32,
}

fn display_quantity(item: &GroceryItem) { //told that its borrowed
    println!("Quantity: {:?}", item.quantity);
}

fn display_id(item: &GroceryItem) { //told that its borrowed
    println!("id: {:?}", item.id_number);
}

fn main() {
    let carrots = GroceryItem{
        quantity: 100,
        id_number: 100023,
    };
    display_id(&carrots); //borrowed
    display_quantity(&carrots); //borrowed
}
