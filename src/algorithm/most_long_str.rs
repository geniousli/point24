use std::collections::hash_map::Entry;
use std::collections::HashMap;
pub struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hash: HashMap<String, usize> = HashMap::new();
        let mut max_len: usize = 0;
        let mut pre_mulit: usize = 0;
        for (index, &item) in s.as_bytes().iter().enumerate() {
            match hash.entry(item.to_string()) {
                Entry::Occupied(mut entry) => {
                    pre_mulit = Solution::max(*entry.get(), pre_mulit);
                    max_len = Solution::max(max_len, index - pre_mulit + 1);
                    entry.insert(index + 1);
                }
                Entry::Vacant(entry) => {
                    entry.insert(index + 1);
                    max_len = Solution::max(max_len, index - pre_mulit + 1);
                }
            }
        }
        max_len as i32
    }

    fn max(a: usize, b: usize) -> usize {
        if a > b {
            a
        } else {
            b
        }
    }
}
