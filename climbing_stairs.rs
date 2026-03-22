fn main() {
    struct Solution;
    impl Solution {
        pub fn climb_stairs(n: i32) -> i32 {
            let x = n - 1;
            let y = n - 2;
            x + y
        }
    }

    println!("{}", Solution::climb_stairs(4));
}
