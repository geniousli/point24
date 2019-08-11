use std::iter::Iterator;
use std::vec::Vec;
use std::collections::VecDeque;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::char;
pub struct Solution {}
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash = HashMap::new();
        for i in strs {
            let mut key = i.chars().map(|x| x.to_string()).collect::<Vec<String>>();
            key.sort();
            let i_str:String = key.join("");
            match hash.entry(i_str) {
                Entry::Vacant(v) => {
                    let mut ary = vec![];
                    ary.push(i);
                    v.insert(ary);
                },
                Entry::Occupied(mut v) => {
                    v.get_mut().push(i);
                }
            }
        }
        hash.values().into_iter().map(|x| x.clone()).collect()
    }
}
