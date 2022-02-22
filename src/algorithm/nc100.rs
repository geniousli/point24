// 本题为考试单行多行输入输出规范示例，无需提交，不计分。
use std::char;
use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::vec::Vec;
fn main() {
    // let mut input = String::new();

    let mut input = "xyxyXX".to_string();
    // std::io::stdin().read_line(&mut input).unwrap();

    let chars: Vec<char> = input.chars().collect();
    let mut hash: HashMap<char, i32> = HashMap::default();

    for ch in chars.into_iter() {
        let val = hash.entry(ch).or_insert(0);
        *val += 1;
    }

    let mut counter_hash: HashMap<i32, Vec<char>> = HashMap::with_capacity(hash.len());
    for (k, v) in hash.into_iter() {
        let vec = counter_hash.entry(v).or_insert(vec![]);
        (*vec).push(k);
        (*vec).sort_by(|a, b| {
            if (b.is_lowercase() && a.is_lowercase()) || (a.is_uppercase() && b.is_uppercase()) {
                a.cmp(b)
            } else {
                if a.is_uppercase() {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        });
    }

    let mut counters: Vec<i32> = counter_hash.keys().map(|i| *i).collect();
    counters.sort_by(|a, b| b.cmp(a));

    let mut output: String = String::default();
    for key in counters.iter() {
        if let Some(ref vals) = counter_hash.get(&key) {
            for v in vals.iter() {
                output.push_str(format!("{}:{};", v, key).as_str());
            }
        }
    }

    println!("{}\n", output);
}
