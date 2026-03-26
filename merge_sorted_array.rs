struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut arr: Vec<i32> = vec![];

        let mut iterating: bool = true;
        while iterating {
            if (nums1[i] > nums2[j]) {
                arr.push[nums1[i]];
                i += 1;
            }
        }

        println!("{:?}", arr)
    }
}

fn main() {
    let mut nums1 = vec![1, 3, 5, 7];
    let mut nums2 = vec![2, 4, 6, 8];

    Solution::merge(&mut nums1, 4, &mut nums2, 3);
}
