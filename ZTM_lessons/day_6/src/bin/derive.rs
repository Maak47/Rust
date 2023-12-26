#[derive(Debug, Clone, Copy)] // need to set here as well because we Position is used in Employee 
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)] // error if some keyword is not used 
 // dont clone and copy in struct with more than 4-5 fields
 // itll take unncessary/expensive copies automatically
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print_employees(emp: Employee){
    println!("{:?}", emp);
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };
    // println!("{:?}",me); // Debug
    print_employees(me);
    print_employees(me); //makes a new copy of me which goes in the function
}