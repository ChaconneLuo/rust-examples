use std::{
    cell::RefCell,
    ptr,
    rc::{Rc, Weak},
};

#[derive(Debug)]
#[allow(dead_code)]
struct Node<T> {
    value: T,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        println!("raw pointer {:p}", &*self);
    }
}

trait Remove<T> {
    fn free(&self, node: &Node<T>);
}

impl<T> Remove<T> for Node<T> {
    fn free(&self, node: &Node<T>) {
        self.children
            .borrow_mut()
            .retain(|child| !ptr::eq(&**child, &*node));
    }
}

fn main() {
    let left_leaf = Rc::new(Node::<i64> {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let root = Rc::new(Node::<i64> {
        value: 10,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&left_leaf)]),
    });
    println!("root initial {:?}", root);

    *left_leaf.parent.borrow_mut() = Rc::downgrade(&root);

    println!("root {:?}", left_leaf.parent.borrow().upgrade());
    println!("leaf {:?}", root.children.borrow().get(0));

    println!(
        "root rc count after changing root = {}",
        Rc::strong_count(&root)
    );
    println!(
        "root weak count after changing root = {}",
        Rc::weak_count(&root)
    );
    {
        let right_leaf = Rc::new(Node::<i64> {
            value: 15,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        root.children.borrow_mut().push(Rc::clone(&right_leaf));
        *right_leaf.parent.borrow_mut() = Rc::downgrade(&root);

        println!(
            "internal root rc count after changing root = {}",
            Rc::strong_count(&root)
        );
        println!(
            "internal root weak count after changing root = {}",
            Rc::weak_count(&root)
        );

        root.free(&right_leaf);
    }

    println!("children count {}", root.children.borrow().len());
    println!(
        "root rc count after changing root = {}",
        Rc::strong_count(&root)
    );
    println!(
        "root weak count after changing root = {}",
        Rc::weak_count(&root)
    );
}
