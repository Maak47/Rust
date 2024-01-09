fn add_fn(a: i32, b: i32) -> i32 {
    // named function we defined
    a + b
}

fn main() {
    let sum = add_fn(2, 3);
    println!("{:?}", sum);

    let add_long = |a: i32, b: i32| -> i32 {
        //anonymous function long
        a + b
    };

    let add = |a, b| a + b; // anonymous function shorthand
    let sum_new = add(2, 3);
    println!("{:?}", sum_new);
}
