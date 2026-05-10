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
        let mut i = 0;
        let mut j = 0;

        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                result.push(left[i]);
                i += 1;
            } else {
                result.push(right[j]);
                j += 1;
            }
        }

        result.extend_from_slice(&left[i..]);
        result.extend_from_slice(&right[j..]);
        result
    }
}

fn main() {
    let sorted = Solution::merge_sort(vec![5, 2, 4, 6, 1, 3]);
    println!("{:?}", sorted);
}
