mod stack;
use stack::Stack;

fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("Top element is: {:?}", stack.peek());
    println!("Stack size is: {}", stack.size());
    
    println!("Popped element: {:?}", stack.pop());
    println!("Top element is: {:?}", stack.peek());
    println!("Stack size is: {}", stack.size());
}
