// 347. Top K Frequent Elements
//
// Given an integer array nums and an integer k, return the k most
// frequent elements. You may return the answer in any order.
#![allow(dead_code)]

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut data: HashMap<i32, i32> = HashMap::with_capacity(nums.capacity());
        // map numbers to keys and add 1 to value if recurring
        for number in nums {
            data.entry(number).and_modify(|value| *value += 1).or_insert(1);
        }
        // sort by value
        let mut data_sorted = Vec::from_iter(data);
        data_sorted.sort_by(|(_, value_a), &(_, value_b)| value_b.cmp(&value_a));
        // take k entries and put it into vec
        data_sorted
            .into_iter()
            .take(k as usize)
            .map(|(key, _)| key)
            .collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod p0347 {
    #[test]
    fn cases() {
        use crate::Solution;
        assert_eq!(Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1])
    }
}

fn main() {}
