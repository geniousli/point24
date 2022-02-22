use std::mem;
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
// use std::collections::linked_list

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub struct Solution {}

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut new_head = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));
        let mut bigger = &mut new_head;

        loop {
            let sign = bigger.as_ref().map_or(false, |inner| {
                inner.next.as_ref().map_or(false, |n| n.val < x)
            });

            if sign {
                bigger = &mut bigger.as_mut().unwrap().next;
            } else {
                break;
            }
        }
        unsafe {
            let mut smaller: *mut Option<Box<ListNode>> = bigger as *mut Option<Box<ListNode>>;
            smaller = (&mut (&mut *smaller).as_mut().unwrap().next) as *mut Option<Box<ListNode>>;

            while (&*smaller).is_some() {
                loop {
                    let sign = (&*smaller).as_ref().map_or(false, |inner| {
                        inner.next.as_ref().map_or(false, |n| n.val >= x)
                    });
                    if sign {
                        smaller = &mut (&mut *smaller).as_mut().unwrap().next
                            as *mut Option<Box<ListNode>>;
                    } else {
                        break;
                    }
                }

                if (&*smaller)
                    .as_ref()
                    .map_or(false, |inner| inner.next.is_some())
                {
                    let mut target = (&mut *smaller)
                        .as_mut()
                        .map_or(None, |inner| inner.next.take());
                    let tnext = target.as_mut().map_or(None, |inner| inner.next.take());
                    if let Some(ref mut inner) = (&mut *smaller) {
                        mem::replace(&mut inner.next, tnext);
                    } else {
                        break;
                    }
                    if let Some(ref mut inner) = bigger {
                        let btmp = mem::replace(&mut inner.next, None);

                        if let Some(ref mut t_inner) = target {
                            mem::replace(&mut t_inner.next, btmp);
                        }
                        mem::replace(&mut inner.next, target);
                        bigger = &mut bigger.as_mut().unwrap().next;
                    }
                } else {
                    break;
                }
            }
        }

        new_head.unwrap().next
    }

    pub fn walk_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut bigger = &mut head;

        loop {
            if let Some(ref inner) = bigger {
                println!("inner val: {}", inner.val);
            } else {
                break;
            }

            bigger = &mut bigger.as_mut().unwrap().next;
        }

        head
    }
}
