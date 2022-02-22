use std::cmp;
use std::i32;
pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Solution {}
    }

    /**
     * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
     *
     * @param array int整型一维数组
     * @return int整型
     */
    pub fn FindGreatestSumOfSubArray(&self, array: Vec<i32>) -> i32 {
        // write code here

        let mut max = i32::MIN;

        let mut sum = 0;
        for item in array {
            if sum >= 0 {
                sum += item;
            } else {
                sum = item;
            }
            max = cmp::max(sum, max);
        }

        max
    }
}
