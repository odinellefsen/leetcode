struct Solution;

impl Solution {
    pub fn merge_sort(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() <= 1 {
            return nums;
        }

        let mid = nums.len() / 2;
        let left = Self::merge_sort(nums[..mid].to_vec());
        let right = Self::merge_sort(nums[mid..].to_vec());

        Self::merge(left, right)
    }

    fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(left.len() + right.len());
        result.extend(left);
        result.extend(right);
        result
    }
}

fn main() {}
