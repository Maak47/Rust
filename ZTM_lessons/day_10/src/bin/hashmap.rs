// lockers - id, contents (using hashmaps)

use std::collections::HashMap; //importing library

#[derive(Debug)]
struct Contents {
    content: String,
}

fn main() {
    let mut lockers = HashMap::new(); //syntax for defining a variable as a HashMap
    lockers.insert(
        1, //key
        Contents {
            //value
            content: "stuff".to_owned(),
        },
    );
    lockers.insert(
        2, //key
        Contents {
            //value
            content: "shirt".to_owned(),
        },
    );
    lockers.insert(
        3, //key
        Contents {
            //value
            content: "books".to_owned(),
        },
    );
    lockers.insert(
        4, //key
        Contents {
            //value
            content: "gym_shorts".to_owned(),
        },
    );

    for (locker_number, content) in lockers.iter()
    //using .iter to iterate over both key-value => (<key>, value)[in the form of tuple] in arbitrary order
    {
        println!("number: {:?} , content: {:?}", locker_number, content);
    }
}
