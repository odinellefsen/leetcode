fn main() {
    struct Solution;
    impl Solution {
        pub fn climb_stairs(n: i32) -> i32 {
            for i in 0..n {
                println!("{}", i);
            }
            0
        }
    }

    Solution::climb_stairs(10);
}
