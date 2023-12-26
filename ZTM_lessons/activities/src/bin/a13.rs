// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    let elements = vec![10, 20, 30, 40];

    for element in &elements { // ownership will be with the for loop thats why borrowed

    match element {
        30 => println!("thirty"),
        _ => println!("{:?}", element),
    }
    }
    println!("number of elements: {:?}", &elements.len()); // borrowed
}
