extern crate point24;
use point24::algorithm::{
    best_time_to_buy_and_sell_stock, find_first_and_last_position_of_element_in_sorted_array, heap,
    lru, middle_number, most_long_str, most_water, receive_water, search_in_rotated_sorted_array,
};
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
    // let string = "abcabcbb".to_string();
    // let mut value = most_long_str::Solution::length_of_longest_substring(string);
    // println!("value is--- {}", value);
    // let string2 = "dvdfd".to_string();
    // value = most_long_str::Solution::length_of_longest_substring(string2);
    // println!("value is--- {}", value);
    // let string3 = "abba".to_string();
    // value = most_long_str::Solution::length_of_longest_substring(string3);
    // println!("str:  value is--- {}", value);
    // let string4 = "rggtlnpgkqksefchmdaqyhdnatpwbtytbho".to_string();
    // value = most_long_str::Solution::length_of_longest_substring(string4);
    // println!("str:  value is--- {}", value);

    // let num1 = vec![1, 3];
    // let num2 = vec![2];
    // let mut value = middle_number::Solution::find_median_sorted_arrays(num1, num2);
    // println!("str:  value is--- {}", value);
    // let nums1 = vec![1, 2];
    // let nums2 = vec![3, 4];
    // let value = middle_number::Solution::find_median_sorted_arrays(nums1, nums2);
    // println!("str:  value is--- {}", value);
    // let nums1 = vec![];
    // let nums2 = vec![1, 2, 3, 4];
    // let value = middle_number::Solution::find_median_sorted_arrays(nums1, nums2);
    // println!("str:  value is--- {}", value);
    // let ary = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    // let value = most_water::Solution::max_area(ary);
    // println!("str:  value is--- {}", value);
    let ary = vec![1];
    let value =
        find_first_and_last_position_of_element_in_sorted_array::Solution::search_range(ary, 1);
    println!("str:  value is--- {:?}", value);
}
