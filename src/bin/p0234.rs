mod p234 {
    // Definition for singly-linked list.
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }

    struct Solution {}
    impl Solution {
        pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
            if head.is_some() {
                let mut number = String::new();
                let mut head_copy = head.clone();
                while let Some(node) = &head_copy {
                    number = format!("{}{}", number, node.val);
                    head_copy = head_copy.unwrap().next;
                }
                if number.chars().rev().collect::<String>() == number {
                    true
                } else {
                    false
                }
            } else {
                true
            }
        }
    }
    #[test]
    fn some_12() {
        let mut head = ListNode::new(1);
        head = ListNode {
            val: 2,
            next: Some(Box::new(head)),
        };
        assert_eq!(false, Solution::is_palindrome(Some(Box::new(head))));
    }

    #[test]
    fn some_1221() {
        let mut head = ListNode::new(1);
        head = ListNode {
            val: 2,
            next: Some(Box::new(head)),
        };
        head = ListNode {
            val: 2,
            next: Some(Box::new(head)),
        };
        head = ListNode {
            val: 1,
            next: Some(Box::new(head)),
        };
        assert_eq!(true, Solution::is_palindrome(Some(Box::new(head))));
    }

    #[test]
    fn none() {
        assert_eq!(true, Solution::is_palindrome(None));
    }
}

fn main() {}
