// https://leetcode.com/problems/two-sum/
use std::collections::HashMap;

struct Solution;
impl Solution {
    // complexity: O(n) where n = nums.len()
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();

        // pre-process: O(n)
        let mut element_index = HashMap::new();
        (0..n).for_each(|i| {
            element_index.insert(nums[i], i);
        });

        // O(n)
        for i in 0..n {
            let corresponding = target - nums[i];
            if let Some(&j) = element_index.get(&corresponding) {
                if i == j {
                    continue;
                }

                return vec![i as i32, j as i32];
            }
        }

        unreachable!()
    }
}

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), [1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), [0, 1]);
}
