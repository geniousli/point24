use std::cmp;
pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Solution {}
    }

    /**
     * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
     * longest common substring
     * @param str1 string字符串 the string
     * @param str2 string字符串 the string
     * @return string字符串
     */
    pub fn LCS(&self, str1: String, str2: String) -> String {
        // write code here

        let len1 = str1.len();
        let len2 = str2.len();
        let mut dps: Vec<Vec<i32>> = vec![vec![0; len1]; len2];

        let chars1: Vec<char> = str1.chars().collect();
        let chars2: Vec<char> = str2.chars().collect();

        let mut max_x = 0;
        let mut max = 0;
        for y in (0..len2) {
            let c2 = chars2[y];
            for x in (0..len1) {
                let c1 = chars1[x];
                if c1 == c2 {
                    // println!("x: {},y: {}", x, y);
                    dps[y][x] = if y < 1 || x < 1 {
                        0
                    } else {
                        dps[y - 1][x - 1]
                    } + 1;

                    if dps[y][x] > max {
                        max_x = x;
                        max = dps[y][x];
                    }
                } else {
                    dps[y][x] = 0;
                }
            }
        }
        let max = max as usize;
        // println!("max: {}, {}: {:?}", max, max_x, dps);
        str1[(max_x + 1 - max)..=max_x].to_string()
    }
}
