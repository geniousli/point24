use std::cmp;
use std::vec::Vec;
pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Solution {}
    }

    /**
     * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
     *
     *
     * @param A string字符串
     * @return int整型
     */
    pub fn getLongestPalindrome(&self, a: String) -> i32 {
        // write code here
        let s_len = a.len();
        if s_len < 2 {
            return 1;
        }

        let mut dp: Vec<Vec<bool>> = vec![vec![false; s_len]; s_len];

        let mut index = 0;
        let mut max = 1;
        while index < s_len {
            dp[index][index] = true;
            index += 1;
        }

        let chars: Vec<char> = a.chars().collect();
        let mut len = 2;

        while len <= s_len {
            // take care also
            let mut start = 0;

            // println!("len {}", len);
            while start < s_len {
                let end = start + len - 1;
                if end >= s_len {
                    break;
                }
                if chars[start] != chars[end] {
                    dp[start][end] = false;
                } else {
                    if end - start < 3 {
                        // this should care
                        dp[start][end] = true;
                    } else {
                        dp[start][end] = dp[start + 1][end - 1];
                    }
                }
                if dp[start][end] {
                    max = cmp::max(len, max);
                }

                start += 1;
            }
            len += 1;
        }
        max as i32
    }
}
