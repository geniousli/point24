use std::vec::Vec;
use std::slice::Iter;
use std::iter::Iterator;
use std::ops::Range;
use std::iter::Enumerate;
use std::string::String;
use std::ops::AddAssign;
pub struct Solution {}

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut ary = Solution::i_get_permutation(n, k);
        ary.reverse();
        Solution::convert_to_string(n,ary)
    }
    fn convert_to_string(n: i32, ary: Vec<i32>)->String {
        let mut str_ary = (1..(n + 1)).collect::<Vec<i32>>();

        let strs = ary.iter().map(|x| {
            let index: usize  = (x - 1) as usize;
            let v = * str_ary.get(index).unwrap();
            str_ary.remove(index);
            v
        }).collect::<Vec<i32>>();
        let mut r_str = String::new();
        for x in strs.iter(){
            r_str.add_assign(x.to_string().as_str());
        }
        r_str
    }
    fn i_get_permutation(n: i32, k: i32) -> Vec<i32> {
        if k <= 0 || n <= 0 {
            return vec![];
        }
        let value = Solution::n_value(n-1);
        let targets: Vec<i32> = (0..(n +1)).collect();
        let mut ceil = 0;
        for (i, item) in targets[0..n as usize].iter().enumerate() {

            let b = targets.get(i + 1).unwrap();
            if item * value < k && k <= b * value {
                ceil = *b;
                break;
            }
        }
        let mut ary = Solution::i_get_permutation(n -1, k - (ceil -1) * value);
        ary.push(ceil);
        ary
    }

    fn n_value(n: i32) -> i32 {
        let value = (1..(n + 1)).fold(1, |a, b| a * b);
        value
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_n_value() {
        let i_str = Solution::get_permutation(3,3);
        assert_eq!(i_str, "213");
    }
    
}
