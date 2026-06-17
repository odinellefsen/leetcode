/**
 * @param {number} x
 * @return {boolean}
 */
var isPalindrome = function(x) {
    let reversed_num = 0;
    let x_x = x;
    let loop = true;
    while (loop) {
        const last_digit = x_x % 10;
        const new_x = Math.floor(x_x / 10);
        x_x = new_x;
        reversed_num = reversed_num * 10 + last_digit;

        if (new_x === 0) {
            loop = false;
        }
    }
    if (x === reversed_num) {
        return true
    } else {
        return false;
    }
};

const result = isPalindrome(1212121)
console.log(result)