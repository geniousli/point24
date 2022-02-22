use std::cmp;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::default::Default;
use std::iter::Iterator;
use std::str::FromStr;
use std::string::String;

pub struct Solution {}

impl Solution {
    // pub fn length_of_longest_substring(s: String) -> i32 {
    //     let mut max_len = 0;
    //     let mut map: HashMap<char, usize> = HashMap::with_capacity(52);
    //     let mut left = 0;

    //     for (i, ch) in s.chars().enumerate() {
    //         let mut pre_loca = None;
    //         {
    //             if let Some(ref pre_local) = map.get(&ch) {
    //                 pre_loca = Some(*pre_local.clone());
    //             }
    //         }

    //         {
    //             if let Some(pre_loca) = pre_loca {
    //                 max_len = cmp::max(max_len, map.len() as i32);
    //                 for ich in s[left..=pre_loca].chars() {
    //                     map.remove_entry(&ich);
    //                 }
    //                 left = pre_loca + 1;
    //             }
    //         }

    //         map.insert(ch, i);
    //     }

    //     cmp::max(max_len, map.len() as i32)
    // }

    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut string = String::default();
        let mut left = 0;
        let mut max_len: i32 = 0;
        for (i, ch) in s.chars().enumerate() {
            if let Some(index) = string.find(ch) {
                let slen = string.len();
                let (_, remain) = string.split_at(index + 1);
                // println!("string: {:?}, ch: {}, remain: {:?}", string, ch, remain);
                string = String::from_str(&remain).unwrap();
                max_len = cmp::max(max_len, (i - left) as i32);
                left = i - slen + index + 1;
            }
            string.push(ch);
        }

        cmp::max(max_len, string.len() as i32)
    }
}
