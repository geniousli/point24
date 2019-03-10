use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::Iterator;
use std::mem::drop;
use std::rc::Rc;
use std::rc::Weak;
use std::str::FromStr;
use std::{thread, time};

#[derive(Debug)]
pub struct Node {
    val: i64,
    left: Option<Rc<RefCell<Box<Node>>>>,
    right: Option<Rc<RefCell<Box<Node>>>>,
    parent: Option<Weak<RefCell<Box<Node>>>>,
}

impl Node {
    fn new(val: i64) -> Node {
        Node {
            val: val,
            left: None,
            right: None,
            parent: None,
        }
    }
}

pub struct AVL {
    root: Option<Node>
}

impl AVL {
    fn new() -> AVL {
        AVL {root: None}
    }

    fn insert(&mut self, val: i64)  {
        if let Some(ref mut node) = self.root {
            

        }else {
            self.root = Some(Node::new(val));
        }
    }

    fn find(&self, val: i64) -> bool {
    }

    // fn hight(&self)
}

fn main() {
    let mut tree = AVL::new();

}
