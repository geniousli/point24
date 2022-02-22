use std::char;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::iter::Iterator;
use std::vec::Vec;
pub struct Solution {}
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut not_val_index = 0;
        let mut index = 0;
        let mut len = nums.len();
        while(index < len) {
            if nums[index] != val {
                nums[not_val_index] = nums[index];
                not_val_index += 1;
            }
            index += 1;
        }
        not_val_index as i32
    }
}
