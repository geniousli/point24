pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0 as usize;
        if nums.len() < 1 {
            return -1;
        }

        let mut right = (nums.len() - 1) as usize;
        let mut index = -1;
        loop {
            println!("right {}, left: {}", right, left);
            if left > right {
                break;
            }
            if nums[left] < target {
                left += 1;
            } else if nums[right] > target {
                if right < 1 {
                    break;
                } else {
                    right -= 1;
                }
            } else {
                if nums[left] == target {
                    index = left as i32;
                } else if nums[right] == target {
                    index = right as i32;
                }
                break;
            }
        }
        index
    }
}
