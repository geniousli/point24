use std::vec::Vec;
use std::string::String;
use std::str::Chars;
use std::iter::Iterator;

#[derive(Debug)]
pub struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let chars = s.chars().collect::<Vec<char>>();
        let step = if num_rows <= 1 {
            1
        }else {
            2 * num_rows - 2
        };
        let mut max_step = 0;
        let mut ary_index = vec![];
        let char_size = s.len() as i32;
        while max_step < char_size {
            ary_index.push(max_step);
            max_step += step as i32;
        }

        (0..num_rows).map(|index|{
            let i_char_ary: Vec<i32>  =
                if index == 0 || index == num_rows - 1 {
                    ary_index.iter().map(|&item| item + index).collect()
                }else {
                    let mut char_ary = vec![];
                    ary_index.iter().for_each(|item|{
                        let one = item + index;
                        let two = item + 2 * num_rows - index - 2;
                        char_ary.push(one);
                        char_ary.push(two);
                    });
                    char_ary
                };
            let mut i_s = String::new();
            i_char_ary.into_iter().filter(|&item| item < char_size).for_each(|item| {
                let i_str = chars.get(item as usize).unwrap();
                i_s.push(*i_str);
            });
            i_s
        }).collect::<String>()
    }
}
