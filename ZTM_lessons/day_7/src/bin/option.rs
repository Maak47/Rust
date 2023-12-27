struct Survey {
    q1: Option<i32>, //used keyword Option<T> = T is some data and is an enum with two variants
    q2: Option<bool>, // contains Some and None variant
    q3: Option<String>, // Some(T) = T is data, None => no data
}

fn main() {
    let response = Survey {
        q1: None,
        q2: Some(true),
        q3: Some("A".to_owned()),
    };

    match response.q1 {
        Some(ans) => println!("q1: {:?}", ans),
        None => println!("q1: no response"),
    }
    match response.q2 {
        Some(ans) => println!("q2: {:?}", ans),
        None => println!("q1: no response"),
    }
    match response.q3 {
        Some(ans) => println!("q3: {:?}", ans),
        None => println!("q1: no response"),
    }
}
