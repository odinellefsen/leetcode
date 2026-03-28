struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // TODO: this should eventually mutate nums1 in place.
        // Placeholder for future length checks against in-place expectations.
        let expected_len = (m + n) as usize;
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut arr: Vec<i32> = Vec::with_capacity(expected_len);

        let m_len = m as usize;
        let n_len = n as usize;

        // TODO: switch to pointer-from-end strategy for true in-place merge.
        while i < m_len && j < n_len {
            if nums1[i] <= nums2[j] {
                arr.push(nums1[i]);
                i += 1;
            } else {
                arr.push(nums2[j]);
                j += 1;
            }
        }

        while i < m_len {
            arr.push(nums1[i]);
            i += 1;
        }

        while j < n_len {
            arr.push(nums2[j]);
            j += 1;
        }

        nums1[..expected_len].copy_from_slice(&arr);
    }
}

fn main() {
    // TODO: convert this into an in-place nums1 validation example.
    let mut nums1 = vec![1, 3, 5, 7, 0, 0, 0, 0];
    let mut nums2 = vec![2, 4, 6, 8];

    Solution::merge(&mut nums1, 4, &mut nums2, 4);
    println!("merged nums1: {:?}", nums1);
}
