use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::drop;
use std::rc::Rc;
use std::str::FromStr;
use std::{thread, time};
fn main() {
    // let nums: Vec<i64> = std::env::args()
    //     .skip(1)
    //     .map(|x| i64::from_str(x.as_str()).unwrap())
    //     .collect();

    // let res: Vec<_> = nums.iter()
    //     .enumerate()
    //     .filter(|(index, _)| index != 0)
    //     .map(|(index, data)| data)
    //     .collect();
    // for x in res {
    //     println!("{:?}", x);
    // }

    // let seq = Vec::new();
    // point_24(nums, seq);
    let mut lru = LruCache::init(2);
    for i in 0..10 {
        lru.put(i, i);
    }

    // for i in 0..1000000 {
    //     lru.put(i, i);
    // }

    // let ten_millis = time::Duration::from_millis(1000);
    // for i in 2..10 {
    //     // println!("------------------");
    //     lru.put(i, i);
    //     // thread::sleep(ten_millis);
    lru.print();
    // }
}

// fn point_24(ary: Vec<i64>, seq: Vec<String>) -> bool {
//     if ary.len() == 2 {
//         let one = ary[0];
//         let two = ary[1];
//         let result = try_combine_to_24(one, two);
//         let re = result.iter().find(|&data| data.0 == 24);
//         match re {
//             Some(&data) => {
//                 println!("data is {:?}", data.1);
//                 true
//             }
//             _ => false,
//         }
//     } else if ary.len() <= 1 {
//         return false;
//     } else {
//         let re = ary.iter().enumerate().find(|&enum1| {
//             let one_i = enum1.0;
//             let one = enum1.1.clone();
//             let remain = ary.iter()
//                 .enumerate()
//                 .filter(|&data| data.0 != one_i)
//                 .map(|(_, val)| val.clone())
//                 .collect::<Vec<i64>>();
//             match remain.iter().enumerate().find(|&data| {
//                 let two_i = data.0;
//                 let two = data.1.clone();

//                 let combine = try_combine_to_24(one, two);
//                 let last = remain
//                     .iter()
//                     .enumerate()
//                     .filter(|&data| data.0 != two_i)
//                     .map(|(_, val)| val.clone())
//                     .collect::<Vec<i64>>();
//                 match combine.iter().find(|&data| {
//                     let para = last.clone();
//                     para.push(data.0);
//                     let i_seq = seq.clone();
//                     i_seq.push(data.1);
//                     point_24(para, i_seq)
//                 }) {
//                     Some(_) => true,
//                     _ => false,
//                 }
//             }) {
//                 Some(_) => true,
//                 _ => false,
//             }
//         });
//         match re {
//             Some(_) => true,
//             _ => false,
//         }
//     }
// }

fn try_combine_to_24(one: i64, two: i64) -> Vec<(i64, String)> {
    let mut ary = Vec::new();
    ary.push((one + two, format!("{}+{}", one, two)));
    ary.push((one - two, format!("{}-{}", one, two)));
    ary.push((two - one, format!("{}-{}", two, one)));
    ary.push((one * two, format!("{}*{}", one, two)));
    if one > two && two != 0 && one % two == 0 {
        ary.push((one / two, format!("{}/{}", one, two)));
    } else if two > one && one != 0 && two % one == 0 {
        ary.push((two / one, format!("{}/{}", two, one)));
    }
    ary
}

#[derive(Debug)]
struct LinkNode {
    val: i64,
    key: i64,
    next: Option<Rc<RefCell<Box<LinkNode>>>>,
    pre: Option<Rc<RefCell<Box<LinkNode>>>>,
}

// impl Drop for LinkNode {
//     fn drop(&mut self) {
//         println!("drop ed {:?}", self.val);
//     }
// }

#[derive(Debug)]
struct LruCache {
    head: Option<Rc<RefCell<Box<LinkNode>>>>,
    tail: Option<Rc<RefCell<Box<LinkNode>>>>,
    cache: HashMap<i64, Rc<RefCell<Box<LinkNode>>>>,
    cap: i64,
    ref_ary: Vec<Rc<RefCell<Box<LinkNode>>>>,
}

impl LruCache {
    fn init(cap: i64) -> LruCache {
        LruCache {
            head: None,
            tail: None,
            cache: HashMap::new(),
            cap: cap,
            ref_ary: Vec::new(),
        }
    }

    fn put(&mut self, key: i64, val: i64) {
        match self.get(key) {
            Some(data) => {}
            None => {
                let node = Rc::new(RefCell::new(Box::new(LinkNode {
                    key: key,
                    val: val,
                    next: None,
                    pre: None,
                })));
                self.ref_ary.push(Rc::clone(&node));
                if let Some(ref head_rc) = self.head {
                    head_rc.borrow_mut().pre = Some(Rc::clone(&node));
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
                            pre_tail_rc.borrow_mut().next = None;
                            last_tail = Some(Rc::clone(&pre_tail_rc));
                        }
                        tail_rc.borrow_mut().next = None;
                        tail_rc.borrow_mut().pre = None;
                        Rc::downgrade(&tail_rc);
                        drop(tail_rc);
                        self.cache.remove(&tail_rc.borrow_mut().val);
                    }
                    self.tail = last_tail;
                }
            }
        }
    }

    fn get(&mut self, key: i64) -> Option<Rc<RefCell<Box<LinkNode>>>> {
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
                                    pre_rc.borrow_mut().next = None;
                                    last_tail = Some(Rc::clone(&pre_rc));
                                    change_tail = true;
                                }
                                curr_ptr.next = None;
                                curr_ptr.pre = None;
                            } else {
                                let curr_ptr = curr_rc.borrow_mut();
                                let pre = &curr_ptr.pre;
                                let next = &curr_ptr.next;
                                if let Some(ref pre_rc) = *pre {
                                    if let Some(ref next_rc) = *next {
                                        pre_rc.borrow_mut().next = Some(Rc::clone(&next_rc));
                                    } else {
                                        pre_rc.borrow_mut().next = None;
                                    }
                                }
                                if let Some(ref next_rc) = *next {
                                    if let Some(ref pre_rc) = *pre {
                                        next_rc.borrow_mut().pre = Some(Rc::clone(&pre_rc));
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
                        head_rc.borrow_mut().pre = Some(Rc::clone(curr_rc));
                        curr_rc.borrow_mut().pre = None;
                    }
                }
                self.head = Some(Rc::clone(&curr_rc));
                Some(Rc::clone(&curr_rc))
            }
            None => None,
        }
    }

    fn print(&self) {
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
        for ref i in self.ref_ary.iter() {
            println!("ref_count is : {} \n", Rc::strong_count(&i));
            drop(i);
            println!("ref_count is : {} \n", Rc::strong_count(&i));
        }
    }
}
