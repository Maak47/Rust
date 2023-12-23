fn main() {
    let mut i = 3;

    //using loop keyword
    loop {
        println!("{:?}", i);
        i -= 1;  
        if i == 0 {
            i = 1;
            break;
        }
    }
    println!("Done");

    //using while keyword
    while i <= 3{
        println!("{:?}", i);
        i += 1;
    }
}
