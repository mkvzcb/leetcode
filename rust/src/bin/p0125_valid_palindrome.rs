// 125. Valid Palindrome
//
// A phrase is a palindrome if, after converting all uppercase letters into
// lowercase letters and removing all non-alphanumeric characters, it reads
// the same forward and backward. Alphanumeric characters include letters and
// numbers.
//
// Given a string s, return true if it is a palindrome, or false otherwise.

#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let result = s.chars().flat_map(|x| {
            if x.is_alphanumeric() {
                Some(x.to_ascii_lowercase())
            } else {
                None
            }
        });

        result.clone().rev().collect::<String>() == result.collect::<String>()
    }
}

#[cfg(test)]
mod p0125 {
    use crate::Solution;
    #[test]
    fn cases() {
        assert_eq!(Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")), true);
        assert_eq!(Solution::is_palindrome(String::from("race a car")), false);
        assert_eq!(Solution::is_palindrome(String::from(" ")), true);
        assert_eq!(Solution::is_palindrome(String::from("0P")), false);
    }
}

fn main() {}
