// REVISION
// // 1
// fn first_name() {
//     println!("Maak");
// }

// fn last_name() {
//     println!("47");
// }

// fn main() {
//     first_name();
//     last_name();
// }


// // 2
// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn display_result(result: i32){
//     println!("{:?}", result);
// }

// fn main(){
//     let result = add(2, 3);
//     display_result(result);
// }
// // 3a
// fn main() {
//     let message = false;
//     if message == true {
//         println!("hello");
//     } else {
//         println!("goodbye");
//     }
// }

// // 3b
// fn main() {
//     let variable = 4;
//     if variable < 5 {
//         println!("<5");
//     } else if variable == 5 {
//         println!("=5");
//     } else {
//         println!(">5");
//     }
// }

// // 4a 
// fn main() {
//     let condition = true;

//     match condition {
//         true => println!("it's true"),
//         false => println!("it's false"),
//     }
// }

// // 4b 
// fn main() {
//     let integer = 4;
//     match integer {
//         1 => println!("one"),
//         2 => println!("two"),
//         3 => println!("three"),
//         _ => println!("other")
//     }
// }

// // 5 
// fn main() {
//     let mut value = 1;
//     loop {
//         println!("{:?}", value);
   
//         if value == 4{
//             break;
//         }
//         value += 1;
//     }
    
// }

// // 6
// fn main() {
//     let mut value = 5;

//     while value >= 1{
//         println!("{:?}", value);
//         value -= 1;
//     }
//     println!("done!");
// }

// // 7
// enum Color {
//     Red,
//     Orange,
//     Yellow,
//     Green,
//     Blue,
//     Indigo,
//     Violet,
// }


// fn print_color(color: Color){
//     match color {
//         Color::Red => println!("red"),
//         Color::Orange => println!("orange"),
//         Color::Yellow => println!("yellow"),
//         Color::Green => println!("green"),
//         Color::Blue => println!("blue"),
//         Color::Indigo => println!("indigo"),
//         Color::Violet => println!("violet"),
//     }
// }

// fn main(){
//     print_color(Color::Red);
// }

// // 8 
// enum Flavors {
//     Sparkling,
//     Cold,
//     Hot,
// }


// struct Drink {
//     flavor: Flavors,
//     quantity: f64,
// }


// fn print_information(drink: Drink) {
//     match drink.flavor {
//         Flavors::Sparkling => println!("flavor: sparkling"),
//         Flavors::Cold => println!("flavor: cold"),
//         Flavors::Hot => println!("flavor: hot"),
//     }

//     println!("quantity: {:?}", drink.quantity);
// }


// fn main() {
//     let cold_drink = Drink {
//         flavor: Flavors::Cold,
//         quantity: 500.0
//     };
//     print_information(cold_drink);
// }

// // 9
// fn coords() -> (i32, i32) {
//     (2, 4)
// }

// fn main() {
//     let (_x, y) = coords(); //destructured
//     if y > 5 {
//         println!("{:?} greater than 5", y);
//     } else if y == 5 {
//         println!("{:?} equal to 5", y);
//     } else {
//         println!("{:?} smaller than 5", y);
//     }
// }

// // 10
// fn print_condition(is_gt_100: bool) {
//     match is_gt_100 {
//         true => println!("its big"),
//         false => println!("its small")
//     }
// }

// fn main() {
//     let variable = 234;
//     let is_gt_100 = variable > 100;
//     print_condition(is_gt_100);
// }

// // 11
// struct GroceryItem {
//     quantity: i32,
//     id: i32,
// }

// fn display_quantity(item: &GroceryItem) {
//     println!("Quantity: {:?}", item.quantity);
// }

// fn display_id(item: &GroceryItem){
//     println!("ID: {:?}", item.id)
// }

// fn main() {
//     let carrot = GroceryItem{
//         quantity: 21,
//         id: 10023,
//     };
   
//     display_quantity(&carrot);
//     display_id(&carrot);
// }


// // 12
// enum BoxColor {
//     Brown,
//     DampGreen,
//     SandalWood,
// }

// impl BoxColor{
//     fn print(&self) {
//         match self {
//             BoxColor::Brown => println!("color: brown"),
//             BoxColor::DampGreen => println!("color: dampgreen"),
//             BoxColor::SandalWood => println!("color: sandalwood"),
//         }
//     }
// }

// struct Dimensions {
//     height: i32,
//     width: i32,
//     depth: i32,
// }

// impl Dimensions{
//     fn print(&self) {
//         println!("height: {:?}", self.height);
//         println!("width: {:?}", self.width);
//         println!("depth: {:?}", self.depth);
//     }
// }

// struct Box{
//     color: BoxColor,
//     weight: i32,
//     dimensions: Dimensions
// }

// impl Box {
//     fn create_box(weight: i32, color: BoxColor, dimensions: Dimensions) -> Self{
//     Self {
//         weight,
//         color,
//         dimensions,
//     }
//     }

//     fn print_box(&self) {
//         self.color.print();
//       self.dimensions.print();
//       println!("weight: {:?}", self.weight);
//     }
// }


// fn main() {
// let machine_box_dimensions = Dimensions{
//     height: 23,
//     width: 12,
//     depth: 17
// };
// let machine_box = Box::create_box(12, BoxColor::DampGreen, machine_box_dimensions);

// machine_box.print_box();
// }

// 13
// fn main() {
//     let elements = vec![10, 20, 30, 40];

//     for element in &elements {
//     match element {
//         30 => println!("thirty"),
//         _ => println!("{:?}", element)
//     }
//     }
//     println!("number of elements: {:?}", &elements.len());
// }

// 14


// struct Person {
//     age: i32,
//     name: String,
//     color: String,
// }

// impl Person{
   

//     fn new_person(age: i32, name: &str, color: &str) -> Self {
//         Self {
//            age: age,
//            name: String::from(name),
//            color: String::from(color),        }
//     }
//     fn print(&self)  {
//         println!("Hey {:?}, I know your favorite color, its {:?}", self.name, self.color)
//     }
// }


// fn main() {
// let people = vec![
//     Person::new_person(21, "Maak", "Blue"),
//     Person::new_person(8, "Erdea", "Black"),
//     Person::new_person(21, "Bloomy", "Red"),
// ];

// for person in people {
//     if person.age <= 10 {
//         Person::print(&person);
//     }
// }
// }

// // 15

// enum Ticket {
//     BackStage(f64, String),
//     Vip(f64, String),
//     Standard(f64),
// }

// fn main() {
//     let people = vec![
//         Ticket::BackStage(23.55, "Jimmy".to_owned()),
//         Ticket::Vip(20.00, "Jack".to_owned()),
//         Ticket::Standard(15.00),
//     ];

//     for person in people {
//         match person {
//             Ticket::BackStage(price, holder) => println!("BackStage holder: {:?}, price: {:?}", holder, price),
//             Ticket::Vip(price, holder) => println!("Vip holder: {:?}, price: {:?}", holder, price),
//             Ticket::Standard(price) => println!("Standard price: {:?}",price),
//         }
//     }
// }

// 16

// struct Student {
//     name: String,
//     locker: Option<i32>,
// }

// fn main() {
//     let students = vec![
//         Student {
//             name: "Jimmy".to_owned(),
//             locker: Some(12),
//         },
//         Student {
//             name: "Joseph".to_owned(),
//             locker: Some(32),
//         },
//         Student {
//             name: "Johan".to_owned(),
//             locker: None,
//         },
//     ];

//     for student in students {
//         println!("name: {:?}", student.name);
//         match student.locker {
//             Some(locker) => println!("locker number: {:?}", locker),
//             None => println!("no locker used"),
//         }
//     }
// }


// 17

fn main () {

    let string = "What the heck is that rack";

    println!("{:?}",string.to_uppercase());
}