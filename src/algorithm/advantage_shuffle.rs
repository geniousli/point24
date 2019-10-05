pub struct Solution {}
impl Solution {
    pub fn advantage_count(mut a:Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        a.sort();
        let mut result = vec![];
        for x in b.iter() {
            let a_index =
                if let Some((index, val)) =  a.iter().enumerate().find(|item| item.1 > x) {
                    index
                }else{
                    0
                };
            let infer = a.remove(a_index);
            result.push(infer);
        }
        result
    }
}
