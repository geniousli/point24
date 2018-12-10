pub mod point;

use self::point::SelfIter;

pub fn find_result(nums: Vec<i64>) {
    let mut iter = SelfIter {
        x: 0,
        y: 0,
        seq: String::from(""),
        vec: nums,
        combiner: 0,
    };
    for i in iter {
        for y in i {
            for x in y {
                if x.vec[0] == 24 {
                    println!("{:?}", x);
                }
            }
        }
    }
}