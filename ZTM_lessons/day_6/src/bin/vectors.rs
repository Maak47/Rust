#[derive(Debug)] 
struct Test {
    score: i32,
}

fn main() {
    let mut my_scores = vec![ //vec! followed by []<brackets>
        Test {score: 90},
        Test {score: 87},
        Test {score: 70},
        Test {score: 92},
        Test {score: 95},
    ];

    for test in &my_scores {  // can be used with for..in loop
        println!("score = {:?}", test.score);
    }

    my_scores.push(Test { score: 79});
    let sixth_score = &my_scores[5];
    println!("score = {:?}", sixth_score );

    my_scores.pop();

    println!("{:?}", my_scores.len());
}