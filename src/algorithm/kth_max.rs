use std::vec::Vec;

pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Solution {}
    }

    /**
     * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
     *
     * @param a int整型一维数组
     * @param n int整型
     * @param K int整型
     * @return int整型
     */
    pub fn findKth(&self, mut a: &mut Vec<i32>, n: i32, K: i32) -> i32 {
        // write code here
        self.find_range(&mut a, 0, n as usize - 1, K as usize)
    }

    pub fn find_range(&self, mut input: &mut Vec<i32>, left: usize, right: usize, k: usize) -> i32 {
        let split_v = input[left];
        let mut start = left;
        let mut walker = left + 1;
        while walker <= right {
            if input[walker] >= split_v {
                input.swap(walker, start + 1);
                start += 1;
            }
            walker += 1
        }

        input.swap(left, start);

        // println!("after_sort: {:?} left: {}, right: {}, split: {}, k: {}", input, left, right, start, k);
        if start + 1 == k {
            return input[start];
        } else {
            if k > start {
                self.find_range(&mut input, start + 1, right, k)
            } else {
                self.find_range(&mut input, left, start - 1, k)
            }
        }
    }
}
