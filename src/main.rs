use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim() {
        "PUSHINT" => println!("you typed PUSHINT!"),
        _ => println!("you typed something else!"),
    }
}

// PUSHINT 3
// PUSHINT 5
// ADD