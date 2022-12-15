// 542. Subtree of Another Tree
// https://leetcode.com/problems/subtree-of-another-tree/
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

pub fn subtree(
    root: &Option<Rc<RefCell<TreeNode>>>,
    sub_root: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (root, sub_root) {
        (Some(root_node), Some(_)) => {
            root == sub_root
                || subtree(&root_node.borrow().left, &sub_root)
                || subtree(&root_node.borrow().right, &sub_root)
        }
        (None, None) => true,
        _ => false,
    }
}

struct Solution {}

impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        subtree(&root, &sub_root)
    }
}

// todo test cases
fn main() {}
