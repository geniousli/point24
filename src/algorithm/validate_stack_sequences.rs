use std::collections::vec_deque::VecDeque;

struct Solution {}
impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut queue = VecDeque::new();
        let mut poped_index = 0;
        let max_len = pushed.len();

        for item in pushed {
            queue.push_back(item);
            while poped_index < max_len
                && queue
                    .back()
                    .as_ref()
                    .map_or(false, |v| **v == popped[poped_index])
            {
                queue.pop_back();
                poped_index += 1;
            }
        }

        queue.len() == 0
    }
}
