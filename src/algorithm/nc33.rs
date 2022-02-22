use std::mem;
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            val: val,
            next: None,
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Solution {}
    }

    /**
     * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
     *
     * @param pHead1 ListNode类
     * @param pHead2 ListNode类
     * @return ListNode类
     */
    pub fn Merge(
        &self,
        pHead1: Option<Box<ListNode>>,
        pHead2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut one = pHead1;
        let mut two = pHead2;

        let mut newnode = Some(Box::new(ListNode { val: 0, next: None }));
        let mut target = &mut newnode;

        while one.is_some() || two.is_some() {
            if one.is_some() && two.is_some() {
                let one_val = one.as_ref().map(|n| n.val);
                let two_val = two.as_ref().map(|n| n.val);

                if one_val.unwrap() > two_val.unwrap() {
                    target.as_mut().unwrap().next = two.take();
                    target = &mut target.as_mut().unwrap().next;
                    two = target.as_mut().unwrap().next.take();
                } else {
                    target.as_mut().unwrap().next = one.take();
                    target = &mut target.as_mut().unwrap().next;
                    one = target.as_mut().unwrap().next.take();
                }
            } else if one.is_some() {
                target.as_mut().unwrap().next = one.take();
                break;
            } else {
                target.as_mut().unwrap().next = two.take();
                break;
            }
        }

        newnode.unwrap().next
    }
}

// 1 ---> 5, -----> 7
// 2 ----> 3 ----> 6
