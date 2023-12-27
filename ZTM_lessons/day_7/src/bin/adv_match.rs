enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket{
    event: String,
    price: i32,
}

fn main(){
    let n = 3;
    match n {
        3 => println!("three"),
        // _ => println!("number: {:?}", n), // not an optimal way as rust read it as 'ignore [_] after this'
        other => println!("number: {:?}", other), //the variable gets available to use
    }
    let flat = Discount::Flat(20);
    match flat {
        Discount::Flat(20) => println!("Flat 2 % "), // now the data inside the compound variant can be accessed
            // have to put the specific match first because 
            // the below match is applicable for every situation
            // and it will match before the actual condition
        Discount::Flat(amount) => println!("Flat discount of {:?}", amount), // now the data inside the compound variant can be accessed
        _ => (), // ignoring everything else
    }
    
    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50,

    };

    match concert {
        Ticket {price : 50 , event} => println!("event @ 50 = {:?}", event), // match for the price, and access the event.
        Ticket {price, ..} => println!("price = {:?}", price),// (..) means any other field ps. when we dont care to match with other field.
      

    }
}