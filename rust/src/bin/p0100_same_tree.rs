// 100. Same Tree
// https://leetcode.com/problems/same-tree/
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
fn is_same_treenode(q: &Option<Rc<RefCell<TreeNode>>>, p: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (q, p) {
        (Some(q_node), Some(p_node)) => {
            q_node.borrow().val == p_node.borrow().val
                && is_same_treenode(&q_node.borrow().left, &p_node.borrow().left)
                && is_same_treenode(&q_node.borrow().right, &p_node.borrow().right)
        }
        (None, None) => true,
        _ => false,
    }
}
struct Solution {}
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        is_same_treenode(&q, &p)
        // could also just do
        // q == p
    }
}

// todo test cases
fn main() {}
