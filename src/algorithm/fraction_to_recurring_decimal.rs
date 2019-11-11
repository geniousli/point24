use std::option::Option;
use std::vec::Vec;

pub struct Solution {}


impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            String::from("0")
        } else if numerator < 0 || denominator < 0 {
            let result = Solution::ii_fraction_to_decimal((numerator as i64).abs() as u64, (denominator as i64).abs() as u64);
            if numerator < 0 && denominator < 0 {
                result
            }else {
                format!("-{}", result)
            }
        } else {
            Solution::ii_fraction_to_decimal(numerator as u64, denominator as u64)
        }
    }

    pub fn ii_fraction_to_decimal(numerator: u64, denominator: u64) -> String {
        if numerator < denominator {
            let (mut i_ary, sign) = Solution::i_fraction_to_decimal((numerator * 10) as u64, denominator as u64);
            let mut  ary = i_ary.iter().map(|&(one, _)| one.to_string()).collect::<Vec<String>>();
            if let Some(index) = sign {
                ary.insert(index as usize, "(".to_string());
                ary.push(")".to_string());
                format!("{}.{}", 0, ary.join(""))
            }else {
                format!("{}.{}", 0, ary.join(""))
            }
        } else {
            let div = numerator / denominator;
            let remain = numerator % denominator;
            if remain == 0 {
                div.to_string()
            }else {
                let (mut i_ary, sign) = Solution::i_fraction_to_decimal((remain * 10) as u64, denominator as u64);
                let mut ary = i_ary.iter().map(|&(one, _)| one.to_string()).collect::<Vec<String>>();
                if let Some(index) = sign {
                    ary.insert(index as usize, "(".to_string());
                    ary.push(")".to_string());
                    format!("{}.{}", div, ary.join(""))
                }else {
                    format!("{}.{}", div, ary.join(""))
                }
            }

        }
    }

    pub fn i_fraction_to_decimal(mut one: u64, mut two: u64) -> (Vec<(u64, u64)>, Option<usize>) {
        println!("one: {}, two: {}", one, two);
        let mut ary = vec![];
        let mut index: Option<usize> = None;
        loop {
            while one < two {
                one *= 10;
                let tuple = (0, one);
                if let Some(index) = ary.iter().position(|&(one, two)| one == tuple.0 && two == tuple.1) {
                    return (ary, Some(index));
                }
                ary.push(tuple);
            }
            let div  = one / two;
            let remain = one % two;
            if remain == 0 {
                ary.push((div, 0));
                return (ary, None);
            }else {
                let tuple = (div, remain * 10);
                if let Some(index) = ary.iter().position(|&(one, two)| one == tuple.0 && two == tuple.1) {
                    return (ary, Some(index));
                }else {
                    ary.push(tuple);
                    one = remain * 10;
                }
            }
        }

    }

}
