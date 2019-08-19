use std::iter::Iterator;
use std::vec::Vec;
use std::collections::VecDeque;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::Range;
use std::char;
pub struct Solution {}
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        let height = len / 2;
        (0..height).for_each(|x| {
            let i_len = len - x * 2;
            (x..(x + i_len - 1)).for_each(|y| {
                let one = matrix[x][y];
                let two = matrix[y][len - x - 1];
                let three = matrix[len - 1 - x][len - y - 1];
                let four = matrix[len - y - 1][x];
                matrix[y][len - x - 1]= one;
                matrix[len - 1 - x][len - y - 1] = two;
                matrix[len - y - 1][x] = three;
                matrix[x][y] = four;
            })
        })
    }
}

    // len = matrix.size
    // height = (len / 2)
    // (0...height).each do |x|
    //     i_len = len - x * 2
    //     (x...(x + i_len - 1)).each do |y|
    //         one = matrix[x][y]
    //         two = matrix[y][len - x - 1]
    //         three = matrix[len - 1 - x][len - y - 1]
    //         four = matrix[len - y - 1][x]
    //         matrix[y][len - x - 1] = one
    //         matrix[len - 1 - x][len - y - 1] = two
    //         matrix[len - y - 1][x] = three
    //         matrix[x][y] = four
    //     end
    // end
