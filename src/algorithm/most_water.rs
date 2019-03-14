pub struct Solution {}
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut max_value = 0;
        while i < j {
            let min = Solution::min(height[i], height[j]);
            let value = (j - i) as i32 * min;
            if value > max_value {
                max_value = value;
            }
            if height[i] > height[j] {
                j -= 1;
            } else {
                i += 1;
            }
        }
        max_value
    }

    fn min(a: i32, b: i32) -> i32 {
        if a < b {
            a
        } else {
            b
        }
    }
}
