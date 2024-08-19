
#[derive(Debug)]
struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>, // Will be a special case later
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T: Clone> {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    fn push_front(&mut self, element: T) {
        let mut new_node = Box::new(Node {
            element,
            next: self.head.take(),
            prev: None,
        });

        if let Some(ref mut old_head) = self.head {
            old_head.prev = Some(Box::new(Node {
                element: new_node.element.clone(), // Clone because we need to update both
                next: None,
                prev: None,
            }));
        } else {
            self.tail = Some(Box::new(Node {
                element: new_node.element.clone(),
                next: None,
                prev: None,
            }));
        }

        self.head = Some(new_node);
        self.len += 1;
    }

    fn push_back(&mut self, element: T) {
        let mut new_node = Box::new(Node {
            element,
            next: None,
            prev: None,
        });

        if let Some(ref mut old_tail) = self.tail.take() {
            old_tail.next = Some(new_node);
            self.tail = Some(old_tail);
        } else {
            self.head = Some(new_node);
            self.tail = self.head.clone();
        }

        self.len += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut old_head| {
            if let Some(new_head) = old_head.next.take() {
                self.head = Some(new_head);
            } else {
                self.tail = None;
            }
            self.len -= 1;
            old_head.element
        })
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|mut old_tail| {
            if let Some(new_tail) = old_tail.prev.take() {
                self.tail = Some(new_tail);
            } else {
                self.head = None;
            }
            self.len -= 1;
            old_tail.element
        })
    }

    fn len(&self) -> usize {
        self.len
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_back(2);
    list.push_front(0);

    println!("List length: {}", list.len());
    println!("Popped from front: {:?}", list.pop_front());
    println!("Popped from back: {:?}", list.pop_back());
    println!("List length: {}", list.len());
}
