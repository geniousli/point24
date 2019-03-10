use std::borrow::BorrowMut;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
//
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
#[derive(Debug)]
pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut walker = &l1;
        let mut walker2 = &l2;
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut pre: i32 = 0;
        Solution::test_method(&walker, &walker2, &mut head, 0);
        // println!("list: {:?}", new_list);
        head.unwrap().next
    }
    // 因为不能修改， walker 中的，
    fn test_method(
        walker: &Option<Box<ListNode>>,
        walker2: &Option<Box<ListNode>>,
        new_list: &mut Option<Box<ListNode>>,
        pre_value: i32,
    ) {
        if let Some(ref i_walker) = walker {
            if let Some(ref i_walker2) = walker2 {
                let new_value = i_walker.val + i_walker2.val + pre_value;
                if let Some(ref mut i_new_list) = new_list {
                    let ibox = Box::new(ListNode::new(new_value % 10));
                    let mut ii_new_list: &mut Box<ListNode> = i_new_list.borrow_mut();
                    ii_new_list.next = Some(ibox);
                    Solution::test_method(
                        &i_walker.next,
                        &i_walker2.next,
                        &mut ii_new_list.next,
                        new_value / 10,
                    )
                }
            } else {
                let new_value = i_walker.val + 0 + pre_value;
                if let Some(ref mut i_new_list) = new_list {
                    let ibox = Box::new(ListNode::new(new_value % 10));
                    let mut ii_new_list: &mut Box<ListNode> = i_new_list.borrow_mut();
                    ii_new_list.next = Some(ibox);
                    Solution::test_method(
                        &i_walker.next,
                        &walker2,
                        &mut ii_new_list.next,
                        new_value / 10,
                    )
                }
            }
        } else if let Some(ref i_walker2) = walker2 {
            if let Some(ref i_walker) = walker {
                let new_value = i_walker.val + i_walker2.val + pre_value;
                if let Some(ref mut i_new_list) = new_list {
                    let ibox = Box::new(ListNode::new(new_value % 10));
                    let mut ii_new_list: &mut Box<ListNode> = i_new_list.borrow_mut();
                    ii_new_list.next = Some(ibox);
                    Solution::test_method(
                        &i_walker.next,
                        &i_walker2.next,
                        &mut ii_new_list.next,
                        new_value / 10,
                    )
                }
            } else {
                let new_value = i_walker2.val + 0 + pre_value;
                if let Some(ref mut i_new_list) = new_list {
                    let ibox = Box::new(ListNode::new(new_value % 10));
                    let mut ii_new_list: &mut Box<ListNode> = i_new_list.borrow_mut();
                    ii_new_list.next = Some(ibox);
                    Solution::test_method(
                        &walker,
                        &i_walker2.next,
                        &mut ii_new_list.next,
                        new_value / 10,
                    )
                }
            }
        } else if pre_value != 0 {
            if let Some(ref mut i_new_list) = new_list {
                let ibox = Box::new(ListNode::new(pre_value % 10));
                let mut ii_new_list: &mut Box<ListNode> = i_new_list.borrow_mut();
                ii_new_list.next = Some(ibox);
                Solution::test_method(&walker, &walker2, &mut ii_new_list.next, pre_value / 10)
            }
        }
    }
}
