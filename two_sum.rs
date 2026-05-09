use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (i, &n) in nums.iter().enumerate() {
            let need = target - n;
            if let Some(&j) = map.get(&need) {
                return vec![j as i32, i as i32];
            }
            map.insert(n, i);
        }
        vec![]
    }
}

fn main() {
    let idx = Solution::two_sum(vec![2, 7, 11, 15], 9);
    println!("{:?}", idx);
    assert_eq!(idx, vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![-1, -2, -4, -5], -6), vec![1, 2]);
}
