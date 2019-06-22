#[derive(Debug)]
pub struct Solution {}
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let default = vec![-1, -1];
        let mut left = 0;
        if nums.len() < 1 {
            return default;
        }
        let mut right = nums.len() - 1;
        let mut index = -1 as i32;
        let mut middle = 0;

        while left <= right {
            middle = (left + right) / 2;
            println!(
                "middle {}, left: {}, right; {}, valu: {}",
                middle, left, right, nums[middle]
            );
            if nums[middle] == target {
                index = middle as i32;
                break;
            } else {
                if nums[middle] > target {
                    if middle <= 0 {
                        break;
                    }
                    right = middle - 1;
                } else {
                    left = middle + 1;
                }
            }
        }

        println!("middle {}, index: {}", middle, index);

        if index == -1 {
            default
        } else {
            right = index as usize;
            left = index as usize;
            println!(
                "------middle {}, left: {}, right; {}, valu: {}",
                middle, left, right, nums[middle]
            );
            while right < nums.len() - 1 {
                if nums[right + 1] == target {
                    right += 1;
                } else {
                    break;
                }
            }
            while left > 0 {
                if nums[left - 1] == target {
                    left -= 1;
                } else {
                    break;
                }
            }
            vec![left as i32, right as i32]
        }
    }
}
