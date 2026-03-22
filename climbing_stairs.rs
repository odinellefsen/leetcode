fn main() {
    struct Solution;
    impl Solution {
        pub fn climb_stairs(n: i32) -> i32 {
            n - 1 + n - 2
        }
    }

    println!("{}", Solution::climb_stairs(4));
}
