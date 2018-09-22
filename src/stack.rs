 pub struct Stack {
    pub _stack: Vec<i32>,
}


impl Stack {
    pub fn push(&mut self, input:&str)
    {
        let foo= input.parse::<i32>().unwrap();
        self._stack.push( foo );
    }
}
