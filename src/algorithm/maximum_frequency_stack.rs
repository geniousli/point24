use std::collections::hash_map::Entry;
use std::collections::vec_deque::VecDeque;
use std::collections::HashMap;
use std::default::Default;
pub struct FreqStack {
    counter_hash: HashMap<i32, i32>,
    group_hash: HashMap<i32, VecDeque<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    pub fn new() -> Self {
        FreqStack {
            counter_hash: HashMap::new(),
            group_hash: HashMap::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        let mut count_val = self.counter_hash.entry(val).or_insert(0);
        *count_val += 1;
        let counter = *count_val;
        let mut queue = self
            .group_hash
            .entry(counter)
            .or_insert(VecDeque::default());
        (*queue).push_back(val);
    }

    pub fn pop(&mut self) -> i32 {
        if let Some(max_val) = self.counter_hash.values().max() {
            if let Some(ref mut val) = self.group_hash.get_mut(max_val) {
                if let Some(key) = val.pop_back() {
                    let entry = self.counter_hash.entry(key).and_modify(|e| *e -= 1);
                    match entry {
                        Entry::Occupied(data) => {
                            if !data.get() > 0 {
                                self.counter_hash.remove(&key);
                            }
                        }

                        Entry::Vacant(_) => {
                            self.counter_hash.remove(&key);
                        }
                    }
                    key
                } else {
                    0
                }
            } else {
                0
            }
        } else {
            0
        }
    }
}
