// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let students = vec![
        Student {
            name: "Freddy".to_owned(),
            locker: Some(14),
        },
        Student {
            name: "Murray".to_owned(),
            locker: None,
        },
        Student {
            name: "Jessie".to_owned(),
            locker: Some(3),
        },
    ];

    for student in students {
        println!("Name: {:?}", student.name);
        match student.locker {
            Some(number) => println!("locker number: {:?}", number),
            None => println!("No Locker assigned"),
        }
    }
}
