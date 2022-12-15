// Two Sum
// Given an array of integers nums and an integer target, return indices of
// the two numbers such that they add up to target.
//
// You may assume that each input would have exactly one solution, and you
// may not use the same element twice.
//
// You can return the answer in any order.
#![allow(dead_code)]

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut data: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (n, k) in nums.iter().enumerate() {
            if !data.contains_key(&(target - k)) {
                data.insert(*k, n);
            } else {
                let (_, a) = data.get_key_value(&(target - k)).unwrap();
                return vec![*a as i32, n as i32];
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod p0001 {
    use crate::Solution;
    #[test]
    fn cases() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}

fn main() {}