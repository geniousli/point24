use std::vec::Vec;
pub struct Solution {}
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut pre = None;
        let mut result = vec![];
        for item in intervals  {
            let left = item[0];
            let right = item[1];
            if let Some(data) = pre {
                let (i_left, i_right)= data;
                if i_right >= left {
                    let new_left = if i_left < left {
                        i_left
                    }else {
                        left
                    };

                    let new_right = if i_right > right {
                        i_right
                    }else {
                        right
                    };
                    pre = Some((new_left, new_right));
                }else{
                    pre = Some((left, right));
                    result.push(vec![i_left, i_right]);
                }
            }else{
                pre = Some((left, right));
            }
        }
        if let Some(data) = pre{
            let (l, r) = data;
            result.push(vec![l, r]);
        }
        result
    }
}
