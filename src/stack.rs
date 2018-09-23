 pub struct Stack {
    _stack: Vec<i32>,
}

impl Stack {

    pub fn new() -> Stack {
        Stack {
            _stack: Vec::<i32>::new(),
        }
    }

    pub fn push(&mut self, value: i32)
    {
        self._stack.insert(0,value);
    }

    pub fn pop(&mut self) -> Result<i32, &str>
    {
        match self._stack.len() {
            0 => Err("Empty Stack, cant ADD!"),
            _ => Ok(self._stack.remove(0))
        }
    }

    pub fn len(&mut self) -> usize { self._stack.len() }

    pub fn peek_print(&mut self)
    {
        if self._stack.len() == 0 {
            println!("Stack is Empty!");
            return;
        }

        println!("value on top of the stack is: {}", self._stack[0]);
    }
}
