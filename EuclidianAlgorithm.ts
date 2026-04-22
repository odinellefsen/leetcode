function gcd(a: number, b: number): number {
    console.log(a, b);
    if (b > a) {
        [a, b] = [b, a];
    }
    const quotient = Math.floor(a / b);
    const remainder = a - (b * quotient);

    
    if (remainder === 0) {
        console.log("b = ", b)
        return b;
    } else {
        a = b;
        b = remainder;
    }

    gcd(a, b);

    return 0;
}

const d = gcd(1160718174, 316258250);
console.log(`The Greatest Common Divisior is: ${d}`);