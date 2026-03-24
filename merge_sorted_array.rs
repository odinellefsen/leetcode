fn main() {
    struct Solution;
    impl Solution {
        pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
            println!("nums1: {:?}", nums1);
            println!("nums2: {:?}", nums2);
        }

        let mut nums1 = vec![1, 3, 5, 7];
        let m = 3
        let mut nums2 = vec![2, 4, 6]
        let n = 3
        Solution::merge(&mut nums1, m, &mut nums2, n);
    }
}
