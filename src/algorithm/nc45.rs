use std::vec::Vec;
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val: val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    fn new() -> Self {
        Solution {}
    }

    pub fn threeOrders(&self, root: Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
        // write code here
        let mut results = vec![];

        let mut preorder = vec![];
        self.preorder(&root, &mut preorder);

        let mut inorder = vec![];
        self.inorder(&root, &mut inorder);

        let mut afterorder = vec![];
        self.afterorder(&root, &mut afterorder);

        results.push(preorder);
        results.push(inorder);
        results.push(afterorder);
        results
    }

    pub fn preorder(&self, root: &Option<Box<TreeNode>>, mut results: &mut Vec<i32>) {
        if let Some(ref node) = root {
            results.push(node.val);
            self.preorder(&node.left, &mut results);
            self.preorder(&node.right, &mut results);
        }
    }

    pub fn inorder(&self, root: &Option<Box<TreeNode>>, mut results: &mut Vec<i32>) {
        if let Some(ref node) = root {
            self.inorder(&node.left, &mut results);
            results.push(node.val);
            self.inorder(&node.right, &mut results);
        }
    }

    pub fn afterorder(&self, root: &Option<Box<TreeNode>>, mut results: &mut Vec<i32>) {
        if let Some(ref node) = root {
            self.afterorder(&node.left, &mut results);
            self.afterorder(&node.right, &mut results);
            results.push(node.val);
        }
    }
}
