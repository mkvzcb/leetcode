// 1704. Determine if String Halves Are Alike
//
// You are given a string s of even length. Split this string into two halves
// of equal lengths, and let a be the first half and b be the second half.
//
// Two strings are alike if they have the same number of vowels
// ('a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U').
// Notice that s contains uppercase and lowercase letters.
//
// Return true if a and b are alike. Otherwise, return false.

#![allow(dead_code)]

const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
struct Solution {}
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let (x, y) = s.split_at(s.len() / 2);
        if x.vowel_count() == y.vowel_count() {
            true
        } else {
            false
        }
    }
}

trait Vowels<'a> {
    fn vowel_count(&self) -> usize;
}

impl<'a> Vowels<'a> for &'a str {
    fn vowel_count(&self) -> usize {
        self.chars()
            .filter(|x| VOWELS.iter().any(|vowel| x == vowel))
            .count()
    }
}

#[cfg(test)]
mod p1704 {
    use crate::Solution;
    #[test]
    fn not_alike() {
        assert_eq!(Solution::halves_are_alike(String::from("textbook")), false)
    }
    #[test]
    fn alike() {
        assert_eq!(Solution::halves_are_alike(String::from("book")), true)
    }
}

fn main() {}
