mod singly_linked_list;
use singly_linked_list::LinkedList;

fn main() {
    let mut list = LinkedList::new();

    println!("Is list empty: {:?}", list.is_empty());
    list.append(1);
    list.append(2);
    list.append(3);

    println!("Traversing the linked list:");
    list.traverse();
    
    println!("Peek front: {:?}", list.peek_front());
    
    println!("Popped front: {:?}", list.pop_front());
    
    println!("Traversing the linked list after pop:");
    list.traverse();
}
