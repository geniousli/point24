use std::collections::VecDeque;
struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Solution {}
    }

    /**
     * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
     *
     * @param s string字符串
     * @return bool布尔型
     */
    pub fn isValid(&self, s: String) -> bool {
        // write code here
        let mut stack = VecDeque::default();
        for ch in s.chars() {
            match ch {
                '(' | '{' | '[' => stack.push_back(ch),
                ')' => {
                    if !stack.pop_back().map_or(false, |v| v != '(') {
                        return false;
                    }
                }
                '}' => {
                    if !stack.pop_back().map_or(false, |v| v != '{') {
                        return false;
                    }
                }
                ']' => {
                    if !stack.pop_back().map_or(false, |v| v != '[') {
                        return false;
                    }
                }
                _ => {}
            }
        }
        stack.len() == 0
    }
}
