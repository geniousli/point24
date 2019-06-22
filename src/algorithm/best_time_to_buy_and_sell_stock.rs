pub struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 {
            return 0;
        }

        let mut pre_min = prices[0];
        let mut max = 0;

        for &item in prices[1..].iter() {
            if item > pre_min {
                if item - pre_min > max {
                    max = item - pre_min;
                }
            } else {
                pre_min = item;
            }
        }
        max
    }
}
