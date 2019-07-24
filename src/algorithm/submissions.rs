use std::iter::Iterator;
use std::vec::Vec;
use std::collections::VecDeque;
pub struct Solution {}
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut ary: VecDeque<i32> =  VecDeque::with_capacity(triangle.len());

        for &i in &triangle[0] {
            ary.push_back(i);
        }
        let mut pre = 1;
        let mut curr = 0;
        if triangle.len() > 1 {
            for item in &triangle[1..] {
                let last = item.len() - 1;
                for (index, &num) in item.iter().enumerate() {
                    if index < (last as usize) {
                        pre = curr;
                        curr = ary.pop_front().unwrap();

                    }
                    match index {
                        0 => {
                            ary.push_back(num + curr);
                        },
                        _ =>{
                            if index == last {
                                ary.push_back(num + curr);
                            }else
                            {
                                let result = if pre > curr {
                                    curr
                                }else{
                                    pre
                                };
                                ary.push_back(result + num);
                            }
                        },
                        // last => {
                        //                            can't write this, error behave
                        // }

                    }
                }
            }

        }
        ary.iter().fold(i32::max_value(), |a, &b| {
            if a > b{
                b
            }else {
                a
            }
        })
    }
}
