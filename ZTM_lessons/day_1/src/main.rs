fn main() {
    // //Variables
    // let two = 2;
    // // let hello = "hello";
    // // let j = 'j';
    // // let my_half = 0.5;
    // // let mut my_name = "Maak";
    // // let quit_program = false;
    // // let your_half = my_half;
    // println!("{:?}", two);

    //Use of function
    let x = add(1, 1);
    let y = add(3, 0);
    let z = add(x, y);
    println!("hello");
    println!("{:?}", z);

    // //if,else, else if statements

    let a = 99;

    // //if..else
    if a > 99 {
        println!("Big Number ");
    } else {
        println!("small Number");
    }

    // //if..if..else..else
    if a > 99 {
        if a > 200 {
            println!("Huge Number");
        } else {
            println!("Big Number");
        }
    } else {
        println!("small number");
    }

    // //if..else if..else

    if a > 200 {
        println!("Huge Number");
    } else if a > 99 {
        println!("Big Number");
    } else {
        println!("small Number");
    }

    //Iterations
    //loop

    let mut a = 0;
    let mut b = 0;

    loop {
        if a == 5 {
            break; //exits the loop
        }
        println!("{:?}", a);
        a = a + 1;
    }

    // while

    while b != 5 {
        println!("{:?}", b);
        b += 1;
    }
}

// Functions

fn add(a: i32, b: i32) -> i32 {
    a + b
}
