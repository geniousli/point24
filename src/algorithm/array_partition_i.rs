use std::iter::Iterator;
use std::vec::Vec;
pub struct Solution {}
impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a, b| b.cmp(a));
        nums.chunks(2).map(|item| {
            if let Some(min)  = item.iter().min() {
                *min
            }else {
                0
            }
        }).collect::<Vec<i32>>().iter().fold(0, |sum, item| sum + item)
    }
}
