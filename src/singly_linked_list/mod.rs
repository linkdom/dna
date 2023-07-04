use std::fmt::Debug;

pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>,
}

impl<T: Debug> LinkedList<T> {

    pub fn new() -> Self {
        LinkedList {head: None, tail: None}
    }

    pub fn append(&mut self, value: T) {
        let new_node = Box::new(Node {value, next: None});

        let raw_node = Box::into_raw(new_node);

        unsafe {

            if let Some(tail) = self.tail {
                (*tail).next = Some(Box::from_raw(raw_node));
            } else {
                self.head = Some(Box::from_raw(raw_node));
            }

            self.tail = Some(raw_node);
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            let old_head = *old_head;
            self.head = old_head.next;

            if self.head.is_none() {
                self.tail = None;
            }

            old_head.value
        })
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn traverse(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("{:?}", node.value);
            current = &node.next;
        }
    }

}

