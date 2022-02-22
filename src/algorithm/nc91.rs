pub struct Solution {}

impl Solution {
    fn new() -> Self {
        Solution {}
    }

    /**
     * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
     * retrun the longest increasing subsequence
     * @param arr int整型一维数组 the array
     * @return int整型一维数组
     */
    pub fn LIS(&self, arr: Vec<i32>) -> Vec<i32> {
        // write code here
        let mut len: Vec<i32> = vec![0; arr.len()];
        let mut lis: Vec<i32> = vec![];
        let mut len = 0;

        for item in vec.iter() {
            if lis.is_empty() {
                vec.push(item);
                len += 1;
            }else {
                if item >  lis[len] {
                    vec.push(item);
                    len += 1;
                }else {
                    let mut l = 0;
                    let mut r = len - 1;

                    let mut pos = 0;
                    while l < r {
                        let m = (l + r) / 2;
                        if vec[m] < item {
                            l = m  + 1;
                            pos = m;
                        }else {
                            r = m - 1;
                        }
                    }
                }
            }

        }
    }
}
