struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut write = m + n - 1;
        let mut i = m - 1;
        let mut j = n - 1;

        while j >= 0 {
            if i >= 0 && nums1[i as usize] > nums2[j as usize] {
                nums1[write as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[write as usize] = nums2[j as usize];
                j -= 1;
            }

            write -= 1;
        }
    }
}

fn main() {
    // TODO: convert this into an in-place nums1 validation example.
    let mut nums1 = vec![1, 3, 5, 7, 0, 0, 0, 0];
    let mut nums2 = vec![2, 4, 6, 8];

    Solution::merge(&mut nums1, 4, &mut nums2, 4);
    println!("merged nums1: {:?}", nums1);
}
