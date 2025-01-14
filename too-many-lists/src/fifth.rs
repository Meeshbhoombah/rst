use std::mem;


pub struct List<T> {
    head: Link<T>,
    tail: Option<&mut Node<T>>
}


type Link<T> = Option<Box<Node<T>>>;


struct Node<T> {
    elem: T,
    next: Link<T>
}


impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None
        }
    }

    pub fn push(& mut self, elem: T) {
        let new_tail = Box::new(Node {
            elem: elem,
            next: None
        });

        let new_tail = match self.tail.take() {
            Some(old_tail) => {
                old_tail.next = Some(new_tail);
                old_tail.next.as_deref_mut();
            }
            None => {
                self.head = Some(new_tail);
                self.head.as_deref_mut();
            }
        };

        self.tail = new_tail;
    }
}

