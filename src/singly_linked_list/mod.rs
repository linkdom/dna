pub struct Node<T> {
    value: T,
    next: Node<T>
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>
}

impl<T> LinkedList<T> {

    pub fn new() -> Self {
        LinkedList {value: None, next: None}
    }

    pub fn append(*mut self, value: T) {
        
    }

}
