// 55. Jump Game
//
// You are given an integer array nums. You are initially positioned at
// the array's first index, and each element in the array represents your
// maximum jump length at that position.
//
//Return true if you can reach the last index, or false otherwise.

// i cant read properly
// did so many solutions by having a wrong idea of the problem

#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let result: i32 = nums.into_iter().fold(1, |mut acc, y| {
            acc = acc - 1;
            if acc >= 0 {
                if acc > y {
                    acc
                } else {
                    y
                }
            } else {
                acc
            }
        });
        if result >= 0 {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod p0055 {
    use crate::Solution;
    #[test]
    fn cases() {
        let mut test_case = (0..9997).rev().collect::<Vec<i32>>();
        assert_eq!(Solution::can_jump(vec![1, 2, 3]), true);
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
        assert_eq!(Solution::can_jump(vec![0]), true);
        assert_eq!(Solution::can_jump(vec![2, 0]), true);
        assert_eq!(Solution::can_jump(vec![1, 1, 1, 0]), true);
        assert_eq!(Solution::can_jump(vec![1, 1, 1, 0]), true);
        assert_eq!(Solution::can_jump(test_case.clone()), true);
        test_case.push(0);
        test_case.push(0);
        assert_eq!(Solution::can_jump(test_case), false);
    }
}

fn main() {}
