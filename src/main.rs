extern crate point24;
use point24::algorithm::{heap, lru};
use point24::problem;

fn main() {
    // let nums: Vec<i64> = std::env::args()
    //     .skip(1)
    //     .map(|x| i64::from_str(x.as_str()).unwrap())
    //     .collect();
    // problem::find_result(nums);

    // let mut ary = vec![10, 20, 0, 15, 26, 80, 90, 100, 1, 2, 3, 17, 10];
    // heap::sort(&mut ary);
    // println!("final ary is: {:?}", ary);

    // let mut cache = lru::LruCache::init(10);
    // for i in 0..1000 {
    //     cache.put(i, i);
    // }
    // cache.print();
}
