fn main() {
    struct Solution;
    impl Solution {
        pub fn climb_stairs(n: i32) -> i32 {
            if (n == 1) {
                return 1;
            }
            if (n == 2) {
                return 2;
            }

            n - 1 + n - 2
        }
    }

    println!("{}", Solution::climb_stairs(2));
}
