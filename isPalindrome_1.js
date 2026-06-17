/**
 * @param {number} x
 * @return {boolean}
 */
var isPalindrome = function(x) {
    const x_str = String(x);
    let a = 0

    const half_length = Math.trunc(x_str.length / 2);
    console.log(half_length);
    
    while (a < x_str.length) {
        console.log("hello bruh", x_str[x_str.length - 1])
        a = 100;
    }
};

const result = isPalindrome(1214)
console.log(result)