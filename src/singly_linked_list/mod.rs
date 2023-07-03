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

}

