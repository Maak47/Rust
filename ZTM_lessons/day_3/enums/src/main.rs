enum Direction { //first letter Capital
    Up, //variant
    Down, //variant
    Left, //variant
    Right //variant
}

fn main() {
    let go = Direction::Left; // accessed using <name of the enum>two-colons(::)<variant>

    match go { //more robust programming using enums and match together
        Direction::Up => println!("go Up"),
        Direction::Down => println!("go Down"),
        Direction::Left => println!("go Left"),
        Direction::Right => println!("go Right"),
    }
}

