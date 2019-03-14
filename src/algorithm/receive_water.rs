pub struct Solution {}
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() <= 0 {
            return 0;
        }
        let result = height.iter().enumerate().max_by_key(|&x| x.1);
        let (max_index, max) = match result {
            Some(tunple) => (tunple.0, *tunple.1),
            None => (0, 0),
        };
        let mut pre_max = 0;
        let mut sum = 0;
        for (index, &item) in height[0..max_index].iter().enumerate() {
            if item > pre_max {
                sum += (item - pre_max) * (max_index - index) as i32;
                pre_max = item;
            }
        }
        pre_max = 0;
        for (index, &item) in height[(max_index + 1)..(height.len())]
            .iter()
            .rev()
            .enumerate()
        {
            if item > pre_max {
                sum += (item - pre_max) * (height.len() - index - max_index - 1) as i32;
                pre_max = item;
            }
        }

        let ary_sum: i32 = height.iter().sum();
        sum - ary_sum + max
    }
}
