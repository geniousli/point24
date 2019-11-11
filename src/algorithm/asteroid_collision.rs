use std::vec::Vec;
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn asteroid_collision(mut asteroids: Vec<i32>) -> Vec<i32> {
        let mut right: VecDeque<i32> = VecDeque::new();
        while asteroids.len() > 0 {
            let new_pop = asteroids.remove(0);
            if let Some(right_value) = right.pop_back() {
                if right_value > 0 && new_pop < 0 {
                    if right_value.abs() < new_pop.abs() {
                        asteroids.insert(0, new_pop);
                    }else if  right_value.abs() >  new_pop.abs(){
                        right.push_back(right_value);
                    }
                }else {
                    right.push_back(right_value);
                    right.push_back(new_pop);
                }
            }else {
                right.push_back(new_pop);
            }
        }
        right.iter().map(|&x| x).collect()
    }
}
