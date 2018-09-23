use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
mod stack;

fn add(stack:&mut stack::Stack )
{
    if stack.len() != 2 {
        println!("ERROR MUST BE ATLEAST TWO NUMBERS ON THE STACK!!");
        return;
    }

    let value1 = stack.pop().unwrap();
    let value2 = stack.pop().unwrap();
    let value2 = value2 + value1;
    stack.push(value2);
}

fn main() {

    let mut stack = stack::Stack::new();
    let f = File::open("test.txt").unwrap();
    let file = BufReader::new(&f);

    for line in file.lines() {
        let l = line.unwrap();
        let split_line:Vec<&str> = l.trim().split(" ").collect::<Vec<&str>>();
        
        match split_line[0].trim() {
        "PUSHINT" => stack.push(split_line[1].parse::<i32>().unwrap()),
        "ADD" => add(&mut stack),
        _ => println!("you typed something else!"),
        }

        stack.peek_print();
    } 
}