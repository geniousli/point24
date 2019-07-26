use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(data) => {
                let (a, b) = Solution::sub_rob(Rc::clone(&data));
                if a > b {
                    a
                }else {
                    b
                }
            }
        }
    }

    fn sub_rob(i_root: Rc<RefCell<TreeNode>>) -> (i32, i32) {
        let root = i_root.borrow_mut();
        if root.left.is_none() && root.right.is_none() {
            (root.val, 0)
        }else {
            let mut l_v = (0, 0);
            let mut r_v = (0, 0);
            if let Some(ref left) = root.left {
                l_v = Solution::sub_rob(Rc::clone(&left));
            }
            if let Some(ref right) = root.right {
                r_v = Solution::sub_rob(Rc::clone(&right));
            }
            let l_max = if l_v.0 > l_v.1 {
                l_v.0
            }else {
                l_v.1
            };

            let r_max = if r_v. 0 > r_v.1 {
                r_v.0
            }else {
                r_v.1
            };

            (l_v.1 + r_v.1 + root.val, l_max + r_max)
        }
    }
}
