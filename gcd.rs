struct Solution;

impl Solution {
    pub fn gcd(mut a: i32, mut b: i32) -> i32 {
        a = a.abs();
        b = b.abs();

        if b == 0 {
            return a;
        }

        while b != 0 {
            let remainder = a % b;
            a = b;
            b = remainder;
        }

        a
    }
}

fn main() {}
