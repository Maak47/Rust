struct Temperature {
    degrees_f: f64,
}

impl Temperature { // using impl for implementing a struct.
    fn freezing() -> Self {
        Self { degrees_f: 32.0}
    } 

    fn boiling() -> Self { // Self implying that we are returning the Temperature.
        Self { degrees_f: 212.0} // Self implying we are accessing the Temperature.
    }

    fn show_temp(&self) { // &self borrows Temperature from itself as its the implementation block of Temperature.
        println!("{:?} degrees F", self.degrees_f);
    }
}

fn main(){
    let hot = Temperature{ // defining a variable to the Temperature structure.
        degrees_f: 99.9
    }; 
    Temperature::show_temp(&hot); // this is how one can use it.

    let cold = Temperature::freezing(); // accessing boiling from the implementation block using <struct name>::<function name>();
    cold.show_temp();

    let boiling = Temperature::boiling(); // accessing boiling from the implementation block using <struct name>::<function name>();
    boiling.show_temp();
    

}