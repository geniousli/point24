use std::iter::Iterator;
use std::vec::Vec;
use std::collections::VecDeque;
pub struct Solution {}
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut right: i32 = (nums.len() - 1) as i32;
        let mut left = 0;
        let mut curr: usize = 0;
        while curr <= right as usize  && curr < nums.len() && right >= 0{
            match nums[curr] {
                0 => {
                    let infer = nums[left];
                    nums[left] = nums[curr];
                    nums[curr] = infer;
                    left += 1;
                    curr += 1;
                },
                2 => {
                    let infer = nums[right as usize];
                    nums[right as usize] = nums[curr];
                    nums[curr] = infer;
                    right -= 1;
                },
                _ => {
                    curr += 1;
                }
            }

        }
    }

}
