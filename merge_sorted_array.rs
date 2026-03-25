struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        println!("m = {m}, n = {n}");
        println!("nums1 = {:?}, nums2 = {:?}", nums1, nums2);
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut arr: Vec<i32> = vec![67, 69];
        while i < nums1.len() {
            println!("{}", nums1[i]);
            arr.push(nums1[i]);
            while j < nums2.len() {
                println!("{}", nums2[j]);
                j += 1;
            }
            j = 0;
            i += 1;
        }
        println!("{:?}", arr)
    }
}

fn main() {
    let mut nums1 = vec![1, 3, 5, 7];
    let mut nums2 = vec![2, 4, 6, 8];

    Solution::merge(&mut nums1, 4, &mut nums2, 3);
}
