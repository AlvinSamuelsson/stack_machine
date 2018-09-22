use std::io;
mod stack;

fn main() {
    
    let mut stack = stack::Stack{ _stack:Vec::<i32>::new() };
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let split_input:Vec<&str> = input.trim().split(" ").collect::<Vec<&str>>();

    //check input
    if split_input.len() != 2 {
        println!("Wrong Input, try: KEYWORD ARGUMENT");
        return;
    }

    match split_input[0].trim() {
        "PUSHINT" => stack.push(split_input[1]),
        _ => println!("you typed something else!"),
    }

    //println!("stack[0] is: {}", stack[0]);
 }

// PUSHINT 3
// PUSHINT 5
// ADD