use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
mod stack;

fn main() {

    let mut stack = stack::Stack::new();
    let f = File::open("test.txt").unwrap();
    let file = BufReader::new(&f);

    for line in file.lines() {
        let l = line.unwrap();
        let split_line:Vec<&str> = l.trim().split(" ").collect::<Vec<&str>>();

        //check input
        if split_line.len() != 2 {
            println!("Wrong Input, try: KEYWORD ARGUMENT");
            return;
        }

        match split_line[0].trim() {
        "PUSHINT" => stack.push(split_line[1]),
        _ => println!("you typed something else!"),
        }

        stack.peek_print();
    } 
}