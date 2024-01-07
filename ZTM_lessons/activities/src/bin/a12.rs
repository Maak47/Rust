// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics


enum BoxColor {
    Brown,
    DampGreen,
    SandalWood,
}

impl BoxColor{
    fn print(&self) {
        match self {
            BoxColor::Brown => println!("color: brown"),
            BoxColor::DampGreen => println!("color: dampgreen"),
            BoxColor::SandalWood => println!("color: sandalwood"),
        }
    }
}

struct Dimensions {
    height: i32,
    width: i32,
    depth: i32,
}

impl Dimensions{
    fn print(&self) {
        println!("height: {:?}", self.height);
        println!("width: {:?}", self.width);
        println!("depth: {:?}", self.depth);
    }
}

struct Box{
    color: BoxColor,
    weight: i32,
    dimensions: Dimensions
}

impl Box {
    fn create_box(weight: i32, color: BoxColor, dimensions: Dimensions) -> Self{
    Self {
        weight,
        color,
        dimensions,
    }
    }

    fn print_box(&self) {
        self.color.print();
      self.dimensions.print();
      println!("weight: {:?}", self.weight);
    }
}


fn main() {
let machine_box_dimensions = Dimensions{
    height: 23,
    width: 12,
    depth: 17
};
let machine_box = Box::create_box(12, BoxColor::DampGreen, machine_box_dimensions);

machine_box.print_box();
}















// enum Color {
//     Red,
//     Orange,
//     Yellow,
//     Green,
//     Blue,
//     Indigo,
//     Violet,   
// }

// impl Color {
//     fn print(&self)  {
//         match self{
//             Color::Red => println!("color: red"),
//             Color::Orange => println!("color: orange"),
//             Color::Yellow => println!("color: yellow"),
//             Color::Green => println!("color: green"),
//             Color::Blue => println!("color: blue"),
//             Color::Indigo => println!("color: indigo"),
//             Color::Violet => println!("color: violet"),
//         }
//     }
//     }
// struct Dimensions {
//     height: f64,
//     width: f64,
//     depth: f64,
// }

// impl Dimensions {
//     fn print(&self) {
//         println!("height: {:?}", self.height);
//         println!("width: {:?}", self.width );
//         println!("depth: {:?}", self.depth);    }
// }

// struct ShippingBox {
//    dimensions: Dimensions,
//     color: Color,
//     weight: f64,
// }
// impl ShippingBox {
//     fn new_box(weight: f64, color: Color, dimensions: Dimensions) -> Self {
//         Self{
//            weight, //NAME OF THE FIELDS ARE SAME SO NO NEED TO PASS THE FIELD-NAME
//             color,
//             dimensions,
//         }
//     }
    
//     fn display_information(&self) {
//         self.color.print();
//         self.dimensions.print();
//         println!("weight: {:?}", self.weight )
       
//     }
// }
// fn main() {
//   let small_dimensions = Dimensions {
//     width: 1.0,
//     height: 2.0,
//     depth: 1.0,
//   };
//   let small_box = ShippingBox::new_box(3.0, Color::Red, small_dimensions);
// small_box.display_information();
// }
