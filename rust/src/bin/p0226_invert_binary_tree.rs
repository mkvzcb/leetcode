// 226. Invert Binary Tree
// https://leetcode.com/problems/invert-binary-tree/
#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

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
            right: None,
        }
    }
}

struct Solution {}
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let left_node = Solution::invert_tree(node.borrow_mut().left.take());
            let right_node = Solution::invert_tree(node.borrow_mut().right.take());
            node.borrow_mut().left = right_node;
            node.borrow_mut().right = left_node;
            Some(node)
        } else {
            None
        }
    }
}

// todo test cases
fn main() {}
