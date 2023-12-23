

struct GroceryItems { //key word struct
    stock: i32, //field
    price: f64, //field
}


fn main() {
    let carrots = GroceryItems{ //have to assign all the data
        stock: 15, 
        price: 5.50,   
    };
    println!("We have {:?} carrots in stock", carrots.stock); // uses dot(.) to access fields
    println!("And Each are priced {:?}", carrots.price);
}