use std::mem;

struct Node<T> {
    next: Option<Box<Node<T>>>,
    prev: Option<*mut Node<T>>,
    element: T,
}

impl<T> Node<T> {
    fn new(element: T) -> Box<Node<T>> {
        Box::new(Node {
            next: None,
            prev: None,
            element,
        })
    }
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: std::ptr::null_mut(),
            len: 0,
        }
    }

    fn push_front(&mut self, element: T) {
        let mut new_node = Node::new(element);

        new_node.prev = None;
        new_node.next = self.head.take();

        let new_node_ptr: *mut _ = &mut *new_node;

        if let Some(old_head) = &mut new_node.next {
            old_head.prev = Some(new_node_ptr);
        } else {
            self.tail = new_node_ptr;
        }

        self.head = Some(new_node);
        self.len += 1;
    }

    fn push_back(&mut self, element: T) {
        let mut new_node = Node::new(element);

        new_node.prev = Some(self.tail);
        new_node.next = None;

        let new_node_ptr: *mut _ = &mut *new_node;

        if !self.tail.is_null() {
            unsafe { (*self.tail).next = Some(new_node) };
        } else {
            self.head = Some(new_node);
        }

        self.tail = new_node_ptr;
        self.len += 1;
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut boxed_node) = current {
            current = boxed_node.next.take();
        }
    }
}
