extern crate point24;
use point24::algorithm::{
    best_time_to_buy_and_sell_stock, find_first_and_last_position_of_element_in_sorted_array, heap,
    lru, middle_number, most_long_str, most_water, receive_water, search_in_rotated_sorted_array, implement_trie_prefix_tree, submissions, house_robber, house_robber_ii,group_anagrams, merge_intervals, sort_colors, simplify_path,
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
    // let ary = vec![1];
    // let value =
    //     find_first_and_last_position_of_element_in_sorted_array::Solution::search_range(ary, 1);
    // println!("str:  value is--- {:?}", value);
    // let mut root = implement_trie_prefix_tree::Trie::new();
    // root.insert("lishaohua".to_string());

    // println!("xxxx {:?}", root);


    // let res = root.search("lishaohua".to_string());
    // println!("result is -----  {:?}", res);

    // let res = root.search("lishao".to_string());
    // println!("result is -----  {:?}", res);

    // let res = root.search("xxx".to_string());
    // println!("result is -----  {:?}", res);

    // let res = root.starts_with("li".to_string());
    // println!("result is -----  {:?}", res);

    // let res = root.starts_with("lishao".to_string());
    // println!("result is -----  {:?}", res);

    // let res = root.starts_with("lishaohuali111".to_string());
    // println!("result is -----  {:?}", res);

    // let res = root.starts_with("lishaohua".to_string());
    // println!("result is -----  {:?}", res);
    // let input =  vec![vec![2],vec![3,4],vec![6,5,7],vec![4,1,8,3]];
    // let value = submissions::Solution::minimum_total(input);
    // println!("value is ---- {}", value);
    // let re = house_robber_ii::Solution::rob(vec![1,2,3,1]);
    // println!("resi s----- {}", re);
    // let mut  problems =  vec![];
    // problems.push(String::from("xxx"));
    // problems.push(String::from("1111"));
    // let ary = vec![vec![1,3], vec![2,6], vec![8,10], vec![15,18]];
    // let result = merge_intervals::Solution::merge(ary);
    // println!("merge is------ {:?}", result);
    // let mut ary = vec![2];
    // sort_colors::Solution::sort_colors(&mut ary);
    // println!("ary: {:?}", ary);
    let string = String::from("//b/../../c/");

    let r_str = simplify_path::Solution::simplify_path(string);
    println!("------ {}", r_str);
}
