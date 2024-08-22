use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct node<T> {
    value: T,
    next: Option<Rc<RefCell<node<T>>>>,
}
#[derive(Debug, Clone)]
struct LinkedList<T> {
    head: Option<Rc<RefCell<node<T>>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }
    fn push_front(&mut self, value: T) {
        let new_node = node {
            value,
            next: self.head.take(),
        };
        self.head = Some(Rc::new(RefCell::new(new_node)));
    }

    fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut vec = Vec::new();
        let mut current = self.head.clone();
        while let Some(node) = current {
            vec.push(node.borrow().value.clone());
            current = node.borrow().next.clone();
        }
        vec
    }
    fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.clone();
        while let Some(node) = current {
            let next = node.borrow_mut().next.take();
            node.borrow_mut().next = prev.take();
            prev = Some(node);
            current = next;
        }
        self.head = prev;
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    println!("Original List: {:?}", list.to_vec());

    list.reverse();

    println!("Reversed List: {:?}", list.to_vec());
}
