struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) -> Vec<i32> {
        // TODO: this should eventually mutate nums1 in place.
        // Placeholder for future length checks against in-place expectations.
        let _expected_len = (m + n) as usize;
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut arr: Vec<i32> = Vec::new();

        // Temporary redeclaration kept during iterative refactor.
        let mut i = 0;
        let mut j = 0;
        let mut arr = Vec::new();

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] <= nums2[j] {
                arr.push(nums1[i]);
                i += 1;
            } else {
                arr.push(nums2[j]);
                j += 1;
            }
        }

        while i < nums1.len() {
            arr.push(nums1[i]);
            i += 1;
        }

        while j < nums2.len() {
            arr.push(nums2[j]);
            j += 1;
        }

        arr
    }
}

fn main() {
    // TODO: convert this into an in-place nums1 validation example.
    let mut nums1 = vec![1, 3, 5, 7];
    let mut nums2 = vec![2, 4, 6, 8];

    let merged_preview = Solution::merge(&mut nums1, 4, &mut nums2, 4);
    println!("preview length: {}", merged_preview.len());
}
