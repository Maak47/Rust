#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    // Result<Ok(data), Err(data)>
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("menu choice not found".to_owned()), // everything else is counted as an error
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("choice: {:?}", choice)
}

fn pick_choice(input: &str) -> Result<(), String> {
    //() is unit-type, means nothing
    let choice: MenuChoice = get_choice(input)?; // ? =< performs automatic match function
    print_choice(&choice); // if its error, then its returned in the function call.
    Ok(())
}

fn main() {
    // let choice: Result<MenuChoice, _> = get_choice("maimenu"); // dont really need the error to pick
    // match choice {
    //     Ok(inner_choice) => print_choice(&inner_choice),
    //     Err(e) => println!("error = {:?}", e),
    // }

    // pick_choice("quit");
    let choice = pick_choice("quit");
    println!("choice value: {:?}", choice);
}
