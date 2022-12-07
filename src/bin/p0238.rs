// 238. Product of Array Except Self
//
// Given an integer array nums, return an array answer such that answer[i] is
// equal to the product of all the elements of nums except nums[i].
//
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit
// integer.
//
// You must write an algorithm that runs in O(n) time and without using the
// division operation.


struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        for n in 0..nums.len() {
            let mut product = 1;
            for l in &nums {
                if *l == nums[n] {

                } else if *l != nums[n] && *l == 0 {
                    product = 0;
                } else {
                    product = product * l;
                }
            }
            result.push(product)
        }
        result
    }
}

#[cfg(test)]
mod p0238 {
    use std::vec;

    #[test]
    fn cases() {
        use crate::Solution;
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}

fn main() {
    println!("{:?}", Solution::product_except_self(vec![0, 0]));
    println!("{:?}", Solution::product_except_self(vec![-1, 1, 0, -3, 3]));
    println!("{:?}", Solution::product_except_self(vec![1, 2, 3, 4]));
}
