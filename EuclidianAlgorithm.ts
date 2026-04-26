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
//const d = gcd(24140, 16762);
//console.log(`The Greatest Common Divisior is: ${d}`);

const a = 140;
const b = 35;
const quotient = Math.floor(a / b);
const remainder = a - (b * quotient);
console.log(`quotient: ${quotient}`);
console.log(`remainder ${remainder}`);
console.log(`dividend: ${quotient * b + remainder}`);