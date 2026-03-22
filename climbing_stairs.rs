fn main() {
    struct Solution;
    impl Solution {
        pub fn climb_stairs(n: i32) -> i32 {
            let mut x = 0;
            if (n - 1 == 0) {
                x = 1;
            } else {
                x = n - 1;
            }

            let mut y = 0;
            if (n - 2 >= 0) {
                y = n - 2;
            } else {
                y = n - 2;
            }
            x + y
        }
    }

    println!("{}", Solution::climb_stairs(2));
}
