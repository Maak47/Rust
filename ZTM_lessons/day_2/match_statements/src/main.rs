/*  Add logic to Program
Similar to if..else
Exhaustive
* All options must be accounted for */
fn main() {
    // boolean
    let a_boolean = false;
    match a_boolean {
        true => println!("this is true"),
        false => println!("this is false"),
    }

    //int
    let integer = 3;
    match integer {
        1 => println!("this is 1"),
        2 => println!("this is 2"),
        3 => println!("this is 3"),
        _ => println!("this is something else"), // underscore(_) is used to tell that everything else other than those required
    }
}

// match vs else..if

/* match will be checked by the compiler
* if a new possibility is added, you will be notified when this occurs
else..if is not checked by the compiler
* If a new possibility is added, your code may contain a bug*/

// Recap
/* * prefer match over else..if when working with a single variable
 * match considers all possibilities
 * more robust code
 * Use underscore (_) to match "anything else"*/
