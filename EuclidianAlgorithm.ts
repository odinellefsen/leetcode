function gcd(a: number, b: number): number {
    if (b > a) {
        [a, b] = [b, a];
    }
    if (b === 0) {
        const absolute_value_of_a = Math.abs(a)
        return absolute_value_of_a;
        
    }
    const quotient = Math.floor(a / b);
    const remainder = a - (b * quotient);
    
    if (remainder === 0) {
        return b;
    } else {
        a = b;
        b = remainder;
        return gcd(a, b);
    }
}

//const d = gcd(1160718174, 316258250);
try {
    const d = gcd(1160718174, 0);
    console.log(`The Greatest Common Divisior is: ${d}`);
} catch(error) {
    console.log(error.message)
}