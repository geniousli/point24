use std::vec::Vec;
use std::iter::Iterator;
pub struct Solution {}


impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() <= 0 {
            return 0;
        }
        if nums.len() <= 3 {
            nums.iter().fold(-1, |a, &b| {
                if a > b {
                    a
                } else {
                    b
                }
            })
        } else {
            let max = Solution::sub_rob(&nums[1..]) ;
            let max2 = Solution::sub_rob(&nums[0..(nums.len() - 1)]);
            if max > max2 {
                max
            } else {
                max2
            }
        }

    }
    pub fn sub_rob(nums: &[i32]) -> i32 {
        if nums.len() <= 0 {
            return 0;
        }
        if nums.len() >= 3 {
            let mut pre1 = *nums.get(0).unwrap();
            let mut pre2 = *nums.get(1).unwrap();
            let mut pre3 = *nums.get(2).unwrap() + pre1;
            for &item in &nums[3..] {
                let tmp = if pre2 > pre1 {
                    pre2
                }else {
                    pre1
                } + item;
                pre1 = pre2;
                pre2 = pre3;
                pre3 = tmp;
            }
            if pre2 > pre3 {
                pre2
            }else {
                pre3
            }
        }else {
            nums.iter().fold(-1, |a, &b| {
                if a > b {
                    a
                }else {
                    b
                }
            })
        }

    }
}
