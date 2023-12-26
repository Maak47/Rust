// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function



struct Person {
    age: i32,
    name: String,
    fav_color: String,
}

impl Person {
    fn new_person(age: i32, name: &str, fav_color: &str) -> Person {
        Person{
           age: age,
           name: String::from(name),
           fav_color: String::from(fav_color),
        }
    }

    fn print_person(person: Person){
        println!("Hey {:?}, I know your favourite color, its {:?}", person.name, person.fav_color);
    }
}

fn main() {
    let people = vec![
        Person::new_person(21, "Zack", "Black"),
        Person::new_person(9, "Tommy", "Blue"),
        Person::new_person(5, "Rosy", "Red"),
    ];

    for person in people {
        if person.age <= 10 {
            Person::print_person(person);
        } 
    }
}
