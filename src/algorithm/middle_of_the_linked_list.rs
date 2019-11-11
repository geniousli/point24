use std::option::Option;

pub struct Solution {}


//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut i_faster = head.clone();
        let mut i_slower = head.clone();
        while i_faster.is_some() {
            i_faster = i_faster.unwrap().next.clone();
            if i_faster.is_some() {
                i_slower = i_slower.unwrap().next.clone();
                i_faster = i_faster.unwrap().next.clone();
            }else {
                break;
            }
        }
        i_slower
    }
}
