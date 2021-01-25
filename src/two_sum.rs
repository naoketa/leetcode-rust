struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let map: HashMap<i32, usize> = nums.iter().enumerate().map(|(i, &num)| (num, i)).collect();

        for (i, v) in nums.iter().enumerate() {
            let looked: i32 = target - v;
            if map.contains_key(&looked) && map[&looked] != i {
                return vec![i as i32, map[&looked] as i32];
            }
        }
        vec![]
    }
}

#[test]
fn case1() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

#[test]
fn case2() {
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}
