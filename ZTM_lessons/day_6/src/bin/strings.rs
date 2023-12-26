struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str){
    println!("name: {:?}", name);
}

fn main(){
    let reciept = vec![
        LineItem {
            name: "Cereal".to_owned(),
            count: 1,
        },
        
        LineItem {
            name: "Carrot".to_owned(),
            count: 5,
        },
        
        LineItem {
            name: String::from("MilkShake"),
            count: 2,
        },
    ];

    for item in reciept {
        print_name(&item.name);
        println!("Count: {:?}",  item.count);
    }

}