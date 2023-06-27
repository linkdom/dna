pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    // Creates a new empty stack
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    // Adds an item to the top of the stack
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    // Removes and returns the top item of the stack
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    // Returns a reference to the top item of the stack without removing it
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    // Returns the number of items in the stack
    pub fn size(&self) -> usize {
        self.items.len()
    }
}
