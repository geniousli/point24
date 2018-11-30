use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::Iterator;
use std::mem::drop;
use std::rc::Rc;
use std::rc::Weak;
use std::str::FromStr;
use std::{thread, time};
fn main() {
    let nums: Vec<i64> = std::env::args()
        .skip(1)
        .map(|x| i64::from_str(x.as_str()).unwrap())
        .collect();

    // let res: Vec<_> = nums.iter()
    //     .enumerate()
    //     .filter(|(index, _)| index != 0)
    //     .map(|(index, data)| data)
    //     .collect();
    // for x in res {
    //     println!("{:?}", x);
    // }
    let mut iter = SelfIter {
        x: 0,
        y: 0,
        seq: String::from(""),
        vec: nums,
        combiner: 0,
    };
    // println!("xxxx is : {:?}", iter);
    // let res = iter.find(|one| {
    //     (*one).find(|&two| two.find(|&three| three.vec[0] == 24).is_some())
    //         .is_some()
    // });
    for i in iter {
        for y in i {
            for x in y {
                if x.vec[0] == 24 {
                    println!("{:?}", x);
                    // return;
                }
            }
        }
    }

    // iter.find(|x| x.find(|y| y.find(|z| z.vec[0] == 24).is_some()).is_some());
    // println!("res is --- {:?}", res);

    // point_24(nums, String::from(""));
    // let mut lru = LruCache::init(10);
    // for i in 0..10 {
    //     lru.put(i, i);
    // }

    // let mut time = 0;
    // while time < 100000 {
    //     lru.put(time, time);
    //     time += 1;
    // }
}

#[derive(Debug)]
struct SelfIter {
    x: usize,
    y: usize,
    vec: Vec<i64>,
    seq: String,
    combiner: i64,
}

impl Iterator for SelfIter {
    type Item = SelfIter;
    fn next(&mut self) -> Option<Self::Item> {
        let len = self.vec.len();
        loop {
            if self.y == self.x {
                self.y += 1;
            }
            if self.x >= len || self.y >= len {
                return None;
            }

            let res = self.try_get_combine();
            if self.combiner >= 4 {
                self.y += 1;
                if self.y >= len {
                    self.x += 1;
                    self.y = 0;
                }
                self.combiner %= 4;
            }

            if res.is_some() {
                return res;
            }
        }
    }
}

impl SelfIter {
    fn try_get_combine(&mut self) -> Option<SelfIter> {
        let mut vec = self
            .vec
            .iter()
            .enumerate()
            .filter(|&data| data.0 != self.x && data.0 != self.y)
            .map(|(_, val)| val.clone())
            .collect::<Vec<i64>>();
        let new_val = match self.combiner {
            0 => {
                if self.y < self.x {
                    None
                } else {
                    Some((
                        self.vec[self.x] + self.vec[self.y],
                        format!("{}+{}", self.vec[self.x], self.vec[self.y]),
                    ))
                }
            }
            1 => Some((
                self.vec[self.x] - self.vec[self.y],
                format!("{}-{}", self.vec[self.x], self.vec[self.y]),
            )),
            2 => {
                if self.y < self.x {
                    None
                } else {
                    Some((
                        self.vec[self.x] * self.vec[self.y],
                        format!("{}*{}", self.vec[self.x], self.vec[self.y]),
                    ))
                }
            }
            3 => {
                if self.vec[self.y] != 0 && self.vec[self.x] % self.vec[self.y] == 0 {
                    Some((
                        self.vec[self.x] / self.vec[self.y],
                        format!("{}/{}", self.vec[self.x], self.vec[self.y]),
                    ))
                } else {
                    None
                }
            }
            _ => None,
        };
        self.combiner += 1;
        match new_val {
            Some((data, sql)) => {
                let new_sql = format!("{}, {}", self.seq.as_str(), sql);
                vec.push(data);
                Some(SelfIter {
                    x: self.x,
                    y: self.y,
                    vec: vec,
                    seq: new_sql,
                    combiner: 0,
                })
            }
            None => None,
        }
    }
}

fn point_24(ary: Vec<i64>, seq: String) -> bool {
    if ary.len() == 2 {
        let one = ary[0];
        let two = ary[1];
        let result = try_combine_to_24(one, two);
        let re = result.iter().find(|&data| data.0 == 24);
        match re {
            Some(ref data) => {
                println!("seq is {}, {}", seq, data.1);
                true
            }
            _ => false,
        }
    } else if ary.len() <= 1 {
        return false;
    } else {
        let re = ary.iter().enumerate().find(|&enum1| {
            let one_i = enum1.0;
            let one = enum1.1.clone();
            let remain = ary
                .iter()
                .enumerate()
                .filter(|&data| data.0 != one_i)
                .map(|(_, val)| val.clone())
                .collect::<Vec<i64>>();
            match remain.iter().enumerate().find(|&data| {
                let two_i = data.0;
                let two = data.1.clone();

                let combine = try_combine_to_24(one, two);
                let last = remain
                    .iter()
                    .enumerate()
                    .filter(|&data| data.0 != two_i)
                    .map(|(_, val)| val.clone())
                    .collect::<Vec<i64>>();
                match combine.iter().find(|&data| {
                    let mut para = last.clone();
                    para.push(data.0);
                    let new_sql = format!("{}, {}", seq, data.1);
                    point_24(para, new_sql)
                }) {
                    Some(_) => true,
                    _ => false,
                }
            }) {
                Some(_) => true,
                _ => false,
            }
        });
        match re {
            Some(_) => true,
            _ => false,
        }
    }
}

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

// impl Drop for LinkNode {
//     fn drop(&mut self) {
//         println!("drop ed {:?}", self.val);
//     }
// }

#[derive(Debug)]
struct LinkNode {
    val: i64,
    key: i64,
    next: Option<Rc<RefCell<Box<LinkNode>>>>,
    pre: Option<Weak<RefCell<Box<LinkNode>>>>,
}

#[derive(Debug)]
struct LruCache {
    head: Option<Rc<RefCell<Box<LinkNode>>>>,
    tail: Option<Rc<RefCell<Box<LinkNode>>>>,
    cache: HashMap<i64, Rc<RefCell<Box<LinkNode>>>>,
    cap: i64,
}

impl LruCache {
    fn init(cap: i64) -> LruCache {
        LruCache {
            head: None,
            tail: None,
            cache: HashMap::new(),
            cap: cap,
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
    }
}
