use std::{cell::RefCell, rc::Rc};
#[derive(Debug, Clone)]
pub struct Node<T> {
    value: Option<T>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

pub trait List<T> {
    fn new() -> Self;
    fn push_back(&self, value: Option<T>);
    fn len(&self) -> usize;
    fn unshift(&self, value: Option<T>);
    fn find(&self, value: Option<T>, eq: fn(args0: &T, args1: &T) -> bool) -> bool;
    // fn remove(&mut self, value: T);
    // fn print(&self);
}

pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> List<T> for LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: Option::Some(Rc::new(RefCell::new(Node {
                value: Option::None,
                next: Option::None,
            }))),
        }
    }

    fn push_back(&self, value: Option<T>) {
        if let Some(ref head) = self.head {
            let mut current: Rc<RefCell<Node<T>>> = Rc::clone(head);
            loop {
                let node_ref = Rc::clone(&current);
                match &(*node_ref).borrow().next {
                    Some(node) => {
                        current = Rc::clone(&node);
                    }
                    None => {
                        break;
                    }
                };
            }

            let new_node: Node<T> = Node {
                value,
                next: Option::None,
            };
            (*current).borrow_mut().next = Option::Some(Rc::new(RefCell::new(new_node)));
        }
    }

    fn unshift(&self, value: Option<T>) {
        if let Some(ref head) = self.head {
            let current: Rc<RefCell<Node<T>>> = Rc::clone(head);
            let new_node: Node<T> = Node {
                value,
                next: (*current).borrow_mut().next.to_owned(),
            };
            (*current).borrow_mut().next = Option::Some(Rc::new(RefCell::new(new_node)));
        }
    }

    fn len(&self) -> usize {
        let mut count: usize = 0;
        if let Some(ref head) = self.head {
            let mut current: Rc<RefCell<Node<T>>> = Rc::clone(head);
            loop {
                let node_ref = Rc::clone(&current);
                match &(*node_ref).borrow().next {
                    Some(node) => {
                        count += 1;
                        current = Rc::clone(&node);
                    }
                    None => {
                        break;
                    }
                };
            }
        }
        count
    }

    fn find(&self, value: Option<T>, eq: fn(args0: &T, args1: &T) -> bool) -> bool {
        true
    }
}
