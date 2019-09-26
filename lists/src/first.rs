
use std::mem;

pub struct List<T> {
    head: Link<T>
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

//enum Link {
//    Empty,
//    More(Box<Node>),
//}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push (&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take()
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {

        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })

    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}