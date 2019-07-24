use std::vec::Vec;
pub struct Solution {}
// Definition for singly-linked list.
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
    // pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {

    // }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test(){
        let mut ary = vec![1,2,3,4,5];
        let head:Option<Box<ListNode>> = ary.iter().rfold(None, |infer, x| {
            let mut new_node = Box::new(ListNode::new(*x));
            new_node.next = infer;
            Some(new_node)
        });
        println!("node is ---{:?}", head);

    }
}
