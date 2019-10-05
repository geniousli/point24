use std::iter::Iterator;
use std::vec::Vec;
use std::collections::VecDeque;
pub struct Solution {}
impl Solution {
    pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        if a.len() < 3 {
            return 0
        }
        let one = a.get(0).unwrap();
        let two = a.get(1).unwrap();
        let mut pre_diff = two - one;
        let mut pre_index = 0;
        let mut pre_end_index = 1;
        let mut result = vec![];
        for index in 2..a.len() {
            let new_diff = a[index] - a[index - 1];
            if pre_diff == new_diff {
                pre_end_index = index;
            }else {
                if pre_end_index - pre_index >= 2 {
                    result.push((pre_index, pre_end_index));
                }
                pre_index = index - 1;
                pre_end_index = index;
                pre_diff = new_diff;
            }
        }
        if pre_end_index == a.len() - 1 && pre_end_index - pre_index >= 2 {
            result.push((pre_index, pre_end_index));
        }

        result.iter().fold(0, |sum, item| {
            let mut start = 2;
            let mut count = 0;
            while item.0 + start <= item.1 {
                count += (item.1 - start) - item.0 + 1;
                start += 1;
            }
            sum + (count as i32)
        })
    }
}
