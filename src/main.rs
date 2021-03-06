extern crate point24;
use point24::algorithm::{
    advantage_shuffle, arithmetic_slices, array_partition_i, asteroid_collision,
    best_time_to_buy_and_sell_stock, find_first_and_last_position_of_element_in_sorted_array,
    fraction_to_recurring_decimal, group_anagrams, heap, house_robber, house_robber_ii,
    implement_trie_prefix_tree, lru, merge_intervals, middle_number, middle_of_the_linked_list,
    most_long_str, most_water, receive_water, search_in_rotated_sorted_array, simplify_path,
    sort_colors, submissions, word_search, word_search_ii, zigzag_conversion,
};
use point24::problem;

struct Person<T> {
    name: T,
    age: i32,
}

impl<T> Person<T> {
    fn say() {
        println!("person -------");
    }
}

impl Person<String> {
    fn say_name() {
        println!("string -------");
    }
}

// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

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
    // let string = String::from("//b/../../c/");

    // let r_str = simplify_path::Solution::simplify_path(string);
    // println!("------ {}", r_str);
    // let ary = vec![1,4,3,2];
    // array_partition_i::Solution::array_pair_sum(ary);
    // let ary = vec![1,2,3,8,9,10];
    // arithmetic_slices::Solution::number_of_arithmetic_slices(ary);

    // let ary = vec![8, -8];
    // let result = asteroid_collision::Solution::asteroid_collision(ary);
    // println!("result: {:?}", result);
    // let ary = fraction_to_recurring_decimal::Solution::fraction_to_decimal(-1, -2147483648);
    // println!("ary: {}", ary);

    // let s = String::from("LEETCODEISHIRING");
    // let result = zigzag_conversion::Solution::convert(s, 4);
    // println!("result --- {}", result);
    // let mut board = vec![
    //     vec!['A', 'B', 'C', 'E'],
    //     vec!['S', 'F', 'C', 'S'],
    //     vec!['A', 'D', 'E', 'E'],
    // ];
    // let word = "ABCCED";
    // let result = word_search::Solution::exist(board, word.to_string());
    // println!("result: #{}", result);

    // let board = vec![
    //     vec!['A', 'B', 'C', 'E'],
    //     vec!['S', 'F', 'E', 'S'],
    //     vec!['A', 'D', 'E', 'E'],
    // ];
    //, "ABCESEEEFS")
    // let word = "ABCESEEEFS";
    // let result = word_search::Solution::exist(board, word.to_string());
    // println!("result: #{}", result);
    // let board = vec![
    //     vec!['a', 'b'],
    //     vec!['c', 'd']
    // ];
    // let word = "cdba";
    // let result = word_search::Solution::exist(board, word.to_string());
    // println!("result: #{}", result);

    let mut board = vec![
        // vec!['o', 'a', 'a', 'n'],
        // vec!['e', 't', 'a', 'e'],
        // vec!['i', 'h', 'k', 'r'],
        // vec!['i', 'f', 'l', 'v'],
        vec!['a', 'b'],
        vec!['a', 'a'],
    ];
    // let mut strs = vec!["aba","baa","bab","aaab","aaa","aaaa","aaba"];
    let mut strs = vec!["aaab", "aaa"];
    let words: Vec<String> = strs.iter().map(|&item| item.to_string()).collect();
    let res = word_search_ii::Solution::find_words(board, words);
    println!("res: #{:?}", res);
}
