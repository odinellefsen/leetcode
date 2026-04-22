function gcd(a: number, b: number): number {
    if (b > a) {
        [a, b] = [b, a];
    }
    const quotient = Math.floor(a / b);
    const remainder = a - (b * quotient);
    
    if (remainder == 0) {
        return b;
    } else {
        a = b;
        b = remainder;
    }

    return 0;
}

gcd(3, 10);