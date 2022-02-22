use std::cmp;
use std::collections::HashMap;
use std::i32;
pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Solution {}
    }

    /**
     * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
     *
     * @param arr int整型一维数组 the array
     * @return int整型
     */
    pub fn maxLength(&self, arr: Vec<i32>) -> i32 {
        // write code here

        let mut index_maps: HashMap<i32, usize> = HashMap::default();
        let mut max = i32::MIN;

        for (i, item) in arr.iter().enumerate() {
            // println!("item: {}", item);
            if let Some(pre_index) = index_maps.get(item) {
                let len = index_maps.len();
                max = cmp::max(max, len as i32);
                for _i in ((i - len)..=*pre_index) {
                    index_maps.remove(&arr[_i]);
                }

            }
            index_maps.insert(*item, i);
        }
        // println!("index_maps.len(): {}", index_maps.len());
        cmp::max(max, index_maps.len() as i32)
    }
}
