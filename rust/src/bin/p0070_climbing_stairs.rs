// 70. Climbing Stairs
// https://leetcode.com/problems/climbing-stairs/

#![allow(dead_code)]

struct Solution {}

fn fibonacci(input: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;
    for _ in 0..input {
        std::mem::swap(&mut first, &mut second);
        second += first;
    }
    second
}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        fibonacci(n)
    }
}

#[cfg(test)]
mod p0070 {
    use crate::Solution;

    #[test]
    fn cases() {
        assert_eq!(Solution::climb_stairs(1), 1);
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(4), 5);
        assert_eq!(Solution::climb_stairs(5), 8);
        assert_eq!(Solution::climb_stairs(6), 13);
        assert_eq!(Solution::climb_stairs(7), 21);
    }
}

fn main() {}
