use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::Iterator;
use std::mem::drop;
use std::rc::Rc;
use std::rc::Weak;
use std::str::FromStr;
use std::{thread, time};

// impl Drop for LinkNode {
//     fn drop(&mut self) {
//         println!("drop ed {:?}", self.val);
//     }
// }

#[derive(Debug)]
pub struct LinkNode {
    val: i64,
    key: i64,
    next: Option<Rc<RefCell<Box<LinkNode>>>>,
    pre: Option<Weak<RefCell<Box<LinkNode>>>>,
}

#[derive(Debug)]
pub struct LruCache {
    head: Option<Rc<RefCell<Box<LinkNode>>>>,
    tail: Option<Rc<RefCell<Box<LinkNode>>>>,
    cache: HashMap<i64, Rc<RefCell<Box<LinkNode>>>>,
    cap: i64,
}

impl LruCache {
    pub fn init(cap: i64) -> LruCache {
        LruCache {
            head: None,
            tail: None,
            cache: HashMap::new(),
            cap: cap,
        }
    }

    pub fn put(&mut self, key: i64, val: i64) {
        match self.get(key) {
            Some(data) => {}
            None => {
                let node = Rc::new(RefCell::new(Box::new(LinkNode {
                    key: key,
                    val: val,
                    next: None,
                    pre: None,
                })));
                if let Some(ref head_rc) = self.head {
                    head_rc.borrow_mut().pre = Some(Rc::downgrade(&node));
                    node.borrow_mut().next = Some(Rc::clone(&head_rc));
                }
                self.head = Some(Rc::clone(&node));

                if self.tail.is_none() {
                    self.tail = Some(Rc::clone(&node));
                }
                self.cache.insert(key, Rc::clone(&node));
                if self.cache.len() as i64 > self.cap {
                    let mut last_tail = None;
                    if let Some(ref tail_rc) = self.tail {
                        if let Some(ref pre_tail_rc) = tail_rc.borrow_mut().pre {
                            if let Some(i_pre_tail_rc) = pre_tail_rc.upgrade() {
                                i_pre_tail_rc.borrow_mut().next = None;
                                last_tail = Some(Rc::clone(&i_pre_tail_rc));
                            }
                        }
                        tail_rc.borrow_mut().next = None;
                        tail_rc.borrow_mut().pre = None;
                        drop(tail_rc);
                        self.cache.remove(&tail_rc.borrow_mut().val);
                    }
                    self.tail = last_tail;
                }
            }
        }
    }

    pub fn get(&mut self, key: i64) -> Option<Rc<RefCell<Box<LinkNode>>>> {
        match self.cache.get(&key) {
            Some(curr_rc) => {
                if let Some(ref head_rc) = self.head {
                    if !Rc::ptr_eq(&head_rc, curr_rc) {
                        let mut change_tail = false;
                        let mut last_tail = None;
                        if let Some(ref tail_rc) = self.tail {
                            if Rc::ptr_eq(&tail_rc, curr_rc) {
                                let mut curr_ptr = curr_rc.borrow_mut();
                                if let Some(ref pre_rc) = curr_ptr.pre {
                                    if let Some(i_pre_rc) = pre_rc.upgrade() {
                                        i_pre_rc.borrow_mut().next = None;
                                        last_tail = Some(Rc::clone(&i_pre_rc));
                                        change_tail = true;
                                    }
                                }
                                curr_ptr.next = None;
                                curr_ptr.pre = None;
                            } else {
                                let curr_ptr = curr_rc.borrow_mut();
                                let pre = &curr_ptr.pre;
                                let next = &curr_ptr.next;
                                if let Some(ref pre_rc) = *pre {
                                    if let Some(i_pre_rc) = pre_rc.upgrade() {
                                        if let Some(ref next_rc) = *next {
                                            i_pre_rc.borrow_mut().next = Some(Rc::clone(&next_rc));
                                        } else {
                                            i_pre_rc.borrow_mut().next = None;
                                        }
                                    }
                                }
                                if let Some(ref next_rc) = *next {
                                    if let Some(ref pre_rc) = *pre {
                                        if let Some(i_pre_rc) = pre_rc.upgrade() {
                                            next_rc.borrow_mut().pre =
                                                Some(Rc::downgrade(&i_pre_rc));
                                        }
                                    } else {
                                        next_rc.borrow_mut().pre = None;
                                    }
                                }
                            }
                        }
                        if change_tail {
                            self.tail = last_tail;
                        }

                        curr_rc.borrow_mut().next = Some(Rc::clone(head_rc));
                        head_rc.borrow_mut().pre = Some(Rc::downgrade(curr_rc));
                        curr_rc.borrow_mut().pre = None;
                    }
                }
                self.head = Some(Rc::clone(&curr_rc));
                Some(Rc::clone(&curr_rc))
            }
            None => None,
        }
    }

    pub fn print(&self) {
        let mut head = self.head.clone();
        loop {
            head = match head {
                Some(ref data) => {
                    let ptr = data.borrow();
                    println!("{:?} \n", ptr.val);
                    match ptr.next {
                        Some(ref i_ptr) => Some(Rc::clone(i_ptr)),
                        None => None,
                    }
                }
                None => break,
            }
        }
    }
}
