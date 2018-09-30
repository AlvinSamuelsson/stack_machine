use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
mod stack;
mod stack_machine;

fn main() {
    let mut stack_machine = stack_machine::StackMachine::new();
    let f = File::open("test.txt").unwrap();
    let file = BufReader::new(&f);

    for line in file.lines() {
        let l = line.unwrap();
        let split_line:Vec<&str> = l.trim().split(" ").collect::<Vec<&str>>();

        match split_line[0].trim() {
        "PUSHINT" => stack_machine.push(split_line[1].parse::<i32>().unwrap()),
        "ADD" => stack_machine.add(),
        _ => println!("you typed something else!"),
        }

        stack_machine.peek_print();
    } 

}