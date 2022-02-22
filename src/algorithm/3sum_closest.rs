use std::char;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::iter::Iterator;
use std::vec::Vec;
pub struct Solution {}
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();

        let mut index = 0;
        let mut len = nums.len();
        let mut closest = nums[0] + num[1] + nums[2];
        let mut min = i32::abs(target - closest);
        while (index < len - 2) {
            let mut left = index + 1;
            let mut right = len - 1;
            let num = nums[index];
            while (left < right) {
                let sum = nums[left] + nums[right] + num;
                if sum > target {
                    right = right - 1;
                    if sum - target < min {
                        min = sum - target;
                        closest = sum;
                    }
                } else {
                    left = left + 1;
                    if target - sum < min {
                        min = target - sum;
                        closest = sum;
                    }
                }
            }
            index += 1;
        }

        closest
    }
}
