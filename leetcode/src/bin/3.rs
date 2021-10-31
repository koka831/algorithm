// https://leetcode.com/problems/longest-substring-without-repeating-characters/
#![allow(unused)]

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let cs = s.chars().collect::<Vec<_>>();
        let n = cs.len();
        let mut appeared = HashMap::new();
        let mut res = 0;

        let mut r = 0;
        for l in 0..n {
            while r < n && !appeared.contains_key(&cs[r]) {
                *appeared.entry(&cs[r]).or_insert(0) += 1;
                r += 1;
            }

            res = res.max(r - l);

            if l == r {
                r += 1;
            } else {
                *appeared.get_mut(&cs[l]).unwrap() -= 1;
                if *appeared.get(&cs[l]).unwrap() == 0 {
                    appeared.remove_entry(&cs[l]);
                }
            }
        }

        res as i32
    }
}

fn main() {
    assert_eq!(Solution::length_of_longest_substring("abcabcbb".into()), 3);
    assert_eq!(Solution::length_of_longest_substring("abc".into()), 3);
    assert_eq!(Solution::length_of_longest_substring("bbbbb".into()), 1);
    assert_eq!(Solution::length_of_longest_substring("pwwkew".into()), 3);
    assert_eq!(Solution::length_of_longest_substring("".into()), 0);
}
