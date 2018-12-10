#[derive(Debug)]
pub struct SelfIter {
    pub x: usize,
    pub y: usize,
    pub vec: Vec<i64>,
    pub seq: String,
    pub combiner: i64,
}

impl Iterator for SelfIter {
    type Item = SelfIter;
    fn next(&mut self) -> Option<Self::Item> {
        let len = self.vec.len();
        loop {
            if self.y == self.x {
                self.y += 1;
            }
            if self.x >= len || self.y >= len {
                return None;
            }

            let res = self.try_get_combine();
            if self.combiner >= 4 {
                self.y += 1;
                if self.y >= len {
                    self.x += 1;
                    self.y = 0;
                }
                self.combiner %= 4;
            }

            if res.is_some() {
                return res;
            }
        }
    }
}

impl SelfIter {
    fn try_get_combine(&mut self) -> Option<SelfIter> {
        let mut vec = self
            .vec
            .iter()
            .enumerate()
            .filter(|&data| data.0 != self.x && data.0 != self.y)
            .map(|(_, val)| val.clone())
            .collect::<Vec<i64>>();
        let new_val = match self.combiner {
            0 => {
                if self.y < self.x {
                    None
                } else {
                    Some((
                        self.vec[self.x] + self.vec[self.y],
                        format!("{}+{}", self.vec[self.x], self.vec[self.y]),
                    ))
                }
            }
            1 => Some((
                self.vec[self.x] - self.vec[self.y],
                format!("{}-{}", self.vec[self.x], self.vec[self.y]),
            )),
            2 => {
                if self.y < self.x {
                    None
                } else {
                    Some((
                        self.vec[self.x] * self.vec[self.y],
                        format!("{}*{}", self.vec[self.x], self.vec[self.y]),
                    ))
                }
            }
            3 => {
                if self.vec[self.y] != 0 && self.vec[self.x] % self.vec[self.y] == 0 {
                    Some((
                        self.vec[self.x] / self.vec[self.y],
                        format!("{}/{}", self.vec[self.x], self.vec[self.y]),
                    ))
                } else {
                    None
                }
            }
            _ => None,
        };
        self.combiner += 1;
        match new_val {
            Some((data, sql)) => {
                let new_sql = format!("{}, {}", self.seq.as_str(), sql);
                vec.push(data);
                Some(SelfIter {
                    x: 0,
                    y: 0,
                    vec: vec,
                    seq: new_sql,
                    combiner: 0,
                })
            }
            None => None,
        }
    }
}




// impl SelfIter {
//     fn try_get_combine(&mut self) -> Option<SelfIter> {
//         let mut vec = self
//             .vec
//             .iter()
//             .enumerate()
//             .filter(|&data| data.0 != self.x && data.0 != self.y)
//             .map(|(_, val)| val.clone())
//             .collect::<Vec<i64>>();
//         let new_val = match self.combiner {
//             0 => {
//                 if self.y < self.x {
//                     None
//                 } else {
//                     Some((
//                         self.vec[self.x] + self.vec[self.y],
//                         format!("{}+{}", self.vec[self.x], self.vec[self.y]),
//                     ))
//                 }
//             }
//             1 => Some((
//                 self.vec[self.x] - self.vec[self.y],
//                 format!("{}-{}", self.vec[self.x], self.vec[self.y]),
//             )),
//             2 => {
//                 if self.y < self.x {
//                     None
//                 } else {
//                     Some((
//                         self.vec[self.x] * self.vec[self.y],
//                         format!("{}*{}", self.vec[self.x], self.vec[self.y]),
//                     ))
//                 }
//             }
//             3 => {
//                 if self.vec[self.y] != 0 && self.vec[self.x] % self.vec[self.y] == 0 {
//                     Some((
//                         self.vec[self.x] / self.vec[self.y],
//                         format!("{}/{}", self.vec[self.x], self.vec[self.y]),
//                     ))
//                 } else {
//                     None
//                 }
//             }
//             _ => None,
//         };
//         self.combiner += 1;
//         match new_val {
//             Some((data, sql)) => {
//                 let new_sql = format!("{}, {}", self.seq.as_str(), sql);
//                 vec.push(data);
//                 Some(SelfIter {
//                     x: 0,
//                     y: 0,
//                     vec: vec,
//                     seq: new_sql,
//                     combiner: 0,
//                 })
//             }
//             None => None,
//         }
//     }
// }

// fn point_24(ary: Vec<i64>, seq: String) -> bool {
//     if ary.len() == 2 {
//         let one = ary[0];
//         let two = ary[1];
//         let result = try_combine_to_24(one, two);
//         let re = result.iter().find(|&data| data.0 == 24);
//         match re {
//             Some(ref data) => {
//                 println!("seq is {}, {}", seq, data.1);
//                 true
//             }
//             _ => false,
//         }
//     } else if ary.len() <= 1 {
//         return false;
//     } else {
//         let re = ary.iter().enumerate().find(|&enum1| {
//             let one_i = enum1.0;
//             let one = enum1.1.clone();
//             let remain = ary
//                 .iter()
//                 .enumerate()
//                 .filter(|&data| data.0 != one_i)
//                 .map(|(_, val)| val.clone())
//                 .collect::<Vec<i64>>();
//             match remain.iter().enumerate().find(|&data| {
//                 let two_i = data.0;
//                 let two = data.1.clone();

//                 let combine = try_combine_to_24(one, two);
//                 let last = remain
//                     .iter()
//                     .enumerate()
//                     .filter(|&data| data.0 != two_i)
//                     .map(|(_, val)| val.clone())
//                     .collect::<Vec<i64>>();
//                 match combine.iter().find(|&data| {
//                     let mut para = last.clone();
//                     para.push(data.0);
//                     let new_sql = format!("{}, {}", seq, data.1);
//                     point_24(para, new_sql)
//                 }) {
//                     Some(_) => true,
//                     _ => false,
//                 }
//             }) {
//                 Some(_) => true,
//                 _ => false,
//             }
//         });
//         match re {
//             Some(_) => true,
//             _ => false,
//         }
//     }
// }

// fn try_combine_to_24(one: i64, two: i64) -> Vec<(i64, String)> {
//     let mut ary = Vec::new();
//     ary.push((one + two, format!("{}+{}", one, two)));
//     ary.push((one - two, format!("{}-{}", one, two)));
//     ary.push((two - one, format!("{}-{}", two, one)));
//     ary.push((one * two, format!("{}*{}", one, two)));
//     if one > two && two != 0 && one % two == 0 {
//         ary.push((one / two, format!("{}/{}", one, two)));
//     } else if two > one && one != 0 && two % one == 0 {
//         ary.push((two / one, format!("{}/{}", two, one)));
//     }
//     ary
// }