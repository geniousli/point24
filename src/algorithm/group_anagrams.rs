use std::char;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::iter::Iterator;
use std::vec::Vec;
pub struct Solution {}
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash = HashMap::new();
        for i in strs {
            let mut key = i.chars().map(|x| x.clone()).collect::<Vec<char>>();
            key.sort();
            let i_str = key.into_iter().fold(String::default(), |mut acc, x| {
                acc.push(x);
                acc
            });
            match hash.entry(i_str) {
                Entry::Vacant(v) => {
                    let mut ary = vec![];
                    ary.push(i);
                    v.insert(ary);
                }
                Entry::Occupied(mut v) => {
                    v.get_mut().push(i);
                }
            }
        }
        hash.values().into_iter().map(|x| x.clone()).collect()
    }
}
