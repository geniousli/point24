use std::iter::Iterator;
use std::vec::Vec;
use std::collections::VecDeque;
pub struct Solution {}
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut path_ary = VecDeque::new();
        let strs:Vec<&str> = path.split('/').collect();
        strs.iter().for_each(|&x| {
            match x {
                ".." => {
                    path_ary.pop_back();
                }
                "."| "" => {
                },
                _ => {
                    path_ary.push_back(x);
                }
            }
        });
        if path_ary.len() != 0 {
            path_ary.iter().fold("".to_string(), |acc, x| {format!("{}/{}", acc, x)})
        }else {
            "/".to_string()
        }
    }
}
