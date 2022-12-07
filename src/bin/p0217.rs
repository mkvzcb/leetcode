// 217. Contains Duplicate
//
// Given an integer array nums, return true if any value appears
// at least twice in the array, and return false if every element is distinct.

#![allow(dead_code)]

use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut data = HashSet::with_capacity(nums.len());
        nums.iter().any(|x| !data.insert(x))
    }
}

mod p0217 {
    #![allow(unused_imports)]
    use crate::Solution;
    #[test]
    fn one_duplicate() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true)
    }
    #[test]
    fn multiple_duplicates() {
        assert_eq!(
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 3, 2, 2]),
            true
        )
    }
    #[test]
    fn no_duplicates() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false)
    }
}

fn main() {}
