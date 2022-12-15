// 104. Maximum Depth of Binary Tree
// https://leetcode.com/problems/maximum-depth-of-binary-tree/
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
fn depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let mut count_left = 1;
        let mut count_right = 1;
        if node.borrow().left.is_some() {
            count_left += depth(node.borrow_mut().left.take());
        }
        if node.borrow().right.is_some() {
            count_right += depth(node.borrow_mut().right.take());
        }

        std::cmp::max(count_left, count_right)
    } else {
        1
    }
}
struct Solution {}
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_some() {
            depth(root)
        } else {
            0
        }
    }
}

// todo test cases
fn main() {}
