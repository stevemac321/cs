use std::ptr;

struct Node<T> {
    next: *mut Node<T>,
    prev: *mut Node<T>,
    element: T,
}

impl<T> Node<T> {
    fn new(element: T) -> *mut Node<T> {
        Box::into_raw(Box::new(Node {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
            element,
        }))
    }
}

struct LinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    fn push_front(&mut self, element: T) {
        unsafe {
            let new_node = Node::new(element);

            if self.head.is_null() {
                // The list is empty
                self.head = new_node;
                self.tail = new_node;
            } else {
                // The list is not empty
                (*new_node).next = self.head;
                (*self.head).prev = new_node;
                self.head = new_node;
            }
            self.len += 1;
        }
    }

    fn push_back(&mut self, element: T) {
        unsafe {
            let new_node = Node::new(element);

            if self.tail.is_null() {
                // The list is empty
                self.head = new_node;
                self.tail = new_node;
            } else {
                // The list is not empty
                (*new_node).prev = self.tail;
                (*self.tail).next = new_node;
                self.tail = new_node;
            }
            self.len += 1;
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        unsafe {
            let mut current = self.head;
            while !current.is_null() {
                let next = (*current).next;
                Box::from_raw(current); // Convert the raw pointer back to a Box and drop it
                current = next;
            }
        }
    }
}