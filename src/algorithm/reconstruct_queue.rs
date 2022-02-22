use std::cmp::Ord;
use std::cmp::PartialOrd;
use std::vec::Vec;

pub struct Solution;
impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut results: Vec<Vec<i32>> = Vec::with_capacity(people.len());

        for person in people {
            let height = person[0];
            let mut limit = person[1];

            let index = if limit == 0 {
                0
            } else {
                let i = results.iter().position(|item| {
                    if item[0] >= height {
                        limit -= 1;
                    }
                    limit == 0
                });
                i.map_or(results.len(), |n| n + 1)
            };

            results.insert(index, person);

            let mut left = index;
            let right = results.len() - 1;

            while left < right {
                let mut i_left = left;
                while i_left + 1 <= right && results[i_left + 1][0] < height {
                    i_left += 1;
                }

                if i_left - left >= 1 {
                    let tmp = results.remove(left);
                    results.insert(i_left, tmp);
                }
                left = i_left + 1;
            }
        }

        results
    }
}
