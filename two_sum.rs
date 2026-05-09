use std::collections::HashMap;

struct Solution;

impl Solution {
    /// Hash map: value → first index seen (two-pass variant).
    pub fn two_sum_with_map(nums: &[i32], target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (i, &n) in nums.iter().enumerate() {
            map.insert(n, i);
        }
        for (i, &n) in nums.iter().enumerate() {
            let need = target - n;
            if let Some(&j) = map.get(&need) {
                if i != j {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::two_sum_with_map(&nums, target)
    }
}

fn main() {
    let idx = Solution::two_sum(vec![2, 7, 11, 15], 9);
    println!("{:?}", idx);
    assert_eq!(idx, vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);

    assert_eq!(Solution::two_sum_with_map(&[2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum_with_map(&[3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum_with_map(&[3, 3], 6), vec![0, 1]);
}
