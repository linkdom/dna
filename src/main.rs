mod queue;

fn main() {
    let mut q = queue::Queue::new();
    
    q.enqueue("Hello");
    q.enqueue("World");
    
    println!("{:?}", q.dequeue()); // Prints Some("Hello")
    println!("{:?}", q.dequeue()); // Prints Some("World")
    println!("{:?}", q.dequeue()); // Prints None
}
