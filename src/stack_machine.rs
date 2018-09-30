use stack;

pub struct StackMachine {
    stack: stack::Stack,
}

impl StackMachine {
    pub fn new() -> StackMachine {
        StackMachine {
            stack: stack::Stack::new(),
        }
    }

    pub fn add(&mut self)
    {
        if self.stack.len() != 2 {
            println!("ERROR MUST BE ATLEAST TWO NUMBERS ON THE STACK!!");
            return;
        }

        let value = self.stack.pop().unwrap() + self.stack.pop().unwrap();
        self.stack.push(value);
    }

    pub fn push(&mut self, value: i32)
    {
        self.stack.push(value);
    }

    pub fn peek_print(&mut self)
    {
        self.stack.peek_print();
    }
}