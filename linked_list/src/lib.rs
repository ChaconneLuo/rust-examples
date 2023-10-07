use std::{cell::RefCell, rc::Rc};

type CoreNode<T> = Rc<RefCell<Node<T>>>;
type LinkNode<T> = Option<CoreNode<T>>;

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    next: LinkNode<T>,
}

pub trait List<T: Copy> {
    fn new() -> Self;
    fn push_back(&mut self, value: T);
    fn len(&self) -> usize;
    fn unshift(&mut self, value: T);
    fn find(&self, value: T, eq: fn(args0: &T, args1: &T) -> bool) -> bool;
    fn pop(&mut self) -> Option<T>;
    fn shift(&mut self) -> Option<T>;
}

pub struct LinkedList<T> {
    length: i64,
    head: LinkNode<T>,
    tail: LinkNode<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> CoreNode<T> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

impl<T: Copy> List<T> for LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            length: 0,
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, value: T) {
        let new_node = Node::new(value);
        new_node.borrow_mut().next = self.head.take();

        match self.tail.take() {
            Some(node) => self.tail = Some(node),
            None => self.tail = Some(new_node.clone()),
        }

        self.length += 1;
        self.head = Some(new_node);
    }

    fn unshift(&mut self, value: T) {
        let new_node = Node::new(value);

        match self.tail.take() {
            Some(node) => {
                node.borrow_mut().next = Some(new_node.clone());
            }
            None => self.head = Some(new_node.clone()),
        }

        self.length += 1;
        self.tail = Some(new_node);
    }

    fn find(&self, value: T, eq: fn(args0: &T, args1: &T) -> bool) -> bool {
        let mut now_node = self.head.clone();
        loop {
            match now_node.take() {
                Some(node) => {
                    let res = eq(&value, &node.borrow().value);
                    if res {
                        return true;
                    } else {
                        now_node = node.borrow().next.clone()
                    }
                }
                None => {
                    return false;
                }
            }
        }
    }

    fn len(&self) -> usize {
        self.length as usize
    }

    fn pop(&mut self) -> Option<T> {
        match self.tail.clone() {
            Some(node) => {
                if self.length != 1 {
                    let mut ptr = self.head.clone();
                    for _index in 1..self.length - 1 {
                        match ptr {
                            Some(node) => {
                                ptr = node.borrow().next.clone();
                            }
                            None => {}
                        }
                    }
                    match ptr.take() {
                        Some(node) => node.borrow_mut().next = None,
                        None => {}
                    }
                    self.length -= 1;
                    self.tail = Some(node.clone());
                    return Some(node.borrow().value);
                } else {
                    self.head = None;
                    self.tail = None;
                    self.length -= 1;
                    return Some(node.borrow().value);
                }
            }
            None => {
                return None;
            }
        }
    }

    fn shift(&mut self) -> Option<T> {
        match self.head.clone() {
            Some(node) => {
                match &node.clone().borrow().next {
                    Some(next_node) => {
                        self.head = Some(next_node.clone());
                    }
                    None => {
                        self.head = None;
                        self.tail = None;
                    }
                }
                self.length -= 1;
                return Some(node.borrow().value);
            }
            None => {
                return None;
            }
        }
    }
}
