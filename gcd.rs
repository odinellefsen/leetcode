struct Solution;

impl Solution {
    pub fn gcd(mut a: i32, mut b: i32) -> i32 {
        a = a.abs();
        b = b.abs();

        if b == 0 {
            return a;
        }

        // Repeatedly replace the pair with divisor and remainder.
        while b != 0 {
            let remainder = a % b;
            a = b;
            b = remainder;
        }

        a
    }
}

fn main() {
    let result = Solution::gcd(48, 18);
    println!("{}", result);

    assert_eq!(result, 6);
    assert_eq!(Solution::gcd(54, 24), 6);
    assert_eq!(Solution::gcd(-42, 56), 14);
}
