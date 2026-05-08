struct Solution;

impl Solution {
    pub fn gcd(mut a: i32, mut b: i32) -> i32 {
        a = a.abs();
        b = b.abs();

        if b == 0 {
            return a;
        }

        0
    }
}

fn main() {}
