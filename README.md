below is a simple lru code in rust & go

rust code 

```rust
use std::collections::HashMap;
use std::mem::drop;
use std::rc::Rc;
use std::rc::Weak;
use std::str::FromStr;
use std::{thread, time};
fn main() {
    let mut lru = LruCache::init(10);
    for i in 0..10 {
        lru.put(i, i);
    }
    let mut time = 0;
    while time < 100000 {
        let mut y = 0;
        while y < 10 {
            lru.put(y, y);
            y += 1;
        }
        time += 1;
    }
}

#[derive(Debug)]
struct LinkNode {
    val: i64,
    key: i64,
    next: Option<Rc<RefCell<Box<LinkNode>>>>,
    pre: Option<Weak<RefCell<Box<LinkNode>>>>,
}

impl Drop for LinkNode {
    fn drop(&mut self) {
        println!("drop ed {:?}", self.val);
    }
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
            cache: HashMap::with_capacity((cap + 1) as usize),
            cap: cap,
        }
    }

    fn put(&mut self, key: i64, val: i64) {
        match self.get(key) {
            Some(data) => {}
            None => {
                if self.cache.len() as i64 <= self.cap - 1 {
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
                } else {
                    let mut new_tail = None;
                    if let Some(ref tail_rc) = self.tail {
                        let mut i_tail_rc = tail_rc.borrow_mut();
                        let old_key = i_tail_rc.key;
                        i_tail_rc.key = key;
                        i_tail_rc.val = val;
                        self.cache.remove(&old_key);
                        self.cache.insert(key, Rc::clone(&tail_rc));
                        if let Some(ref head_rc) = self.head {
                            i_tail_rc.next = Some(Rc::clone(&head_rc));
                            head_rc.borrow_mut().pre = Some(Rc::downgrade(&tail_rc));
                        }
                        self.head = Some(Rc::clone(&tail_rc));
                        if let Some(ref pre_weak) = i_tail_rc.pre {
                            if let Some(ref pre_rc) = pre_weak.upgrade() {
                                pre_rc.borrow_mut().next = None;
                                new_tail = Some(Rc::clone(&pre_rc));
                            }
                        }
                        i_tail_rc.pre = None;
                    }
                    self.tail = new_tail;
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
                                let curr_ptr = curr_rc.borrow_mut();
                                if let Some(ref pre_rc) = curr_ptr.pre {
                                    if let Some(i_pre_rc) = pre_rc.upgrade() {
                                        i_pre_rc.borrow_mut().next = None;
                                        last_tail = Some(Rc::clone(&i_pre_rc));
                                        change_tail = true;
                                    }
                                }
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
                                } else {
                                    if let Some(ref next_rc) = *next {
                                        next_rc.borrow_mut().next = None;
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
                self.head.clone()
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
```

```go

package main

import (
   "fmt"
   "strconv"
)

type DoubleLinkedlistNode struct {
   Key  int
   Val  int
   Pre  *DoubleLinkedlistNode
   Next *DoubleLinkedlistNode
}

type LRUCache struct {
   Cap   int
   Cache map[int]*DoubleLinkedlistNode
   Head  *DoubleLinkedlistNode
   Tail  *DoubleLinkedlistNode
}

func main() {
   obj := Constructor(10)
   for i := 0; i < 10; i++ {
	    obj.Put(i, i)
   }

   for j := 0; j < 100000; j++ {
       for y := 0; y < 10; y ++ {
	    obj.Put(y, y)
       }
   }
  fmt.Println(obj)
}

func Constructor(capacity int) LRUCache {
   return LRUCache{Cap: capacity, Cache: make(map[int]*DoubleLinkedlistNode)}
}

func PrintDoubleLinkedlist(list *DoubleLinkedlistNode) {
   if list == nil {
      return
   }
   str := ""
   for list != nil {
      if len(str) == 0 {
         str += strconv.Itoa(list.Val)
      } else {
         str += "<->"
         str += strconv.Itoa(list.Val)
      }
      list = list.Next
   }
   fmt.Println(str)
}

func (this *LRUCache) Get(key int) int {
   valNode := this.Cache[key]
   if valNode != nil {
      curr := valNode
      // 不是头结点
      if curr != this.Head {
         // 是未结点
         if curr == this.Tail {
            this.Tail = this.Tail.Pre
            this.Tail.Next = nil
         } else {
            curr.Pre.Next = curr.Next
            curr.Next.Pre = curr.Pre
         }
         curr.Next = this.Head
         this.Head.Pre = curr
         curr.Pre = nil
         this.Head = curr

      }
      return valNode.Val
   } else {
      return -1
   }
}


func (this *LRUCache) Put(key int, value int) {
   valNode := this.Cache[key]
   if valNode != nil {
      valNode.Val = value
      this.Get(key)
   } else {
      // 不存在此结点
      tmp := &DoubleLinkedlistNode{Key:key, Val:value}
      if this.Head != nil {
         this.Head.Pre = tmp
      }
      tmp.Next = this.Head
      this.Head = tmp
      if this.Tail == nil {
         this.Tail = tmp
      }
      this.Cache[key] = tmp
      if len(this.Cache) > this.Cap {
         delete(this.Cache, this.Tail.Key)
         this.Tail = this.Tail.Pre
         this.Tail.Next = nil
      }

   }
}
```
