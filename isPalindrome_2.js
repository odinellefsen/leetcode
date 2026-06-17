// time complexity is O(n)
/**
 * @param {number} x
 * @return {boolean}
 */
var isPalindrome = function(x) {
    if (x < 0) {
        return false
    }

    let reversed_num = 0;
    let x_x = x;
    let loop = true;
    const x_str = String(x);
    let i = x_str.length - 1;
    while (loop) {
        const last_digit = x_x % 10;
        const new_x = Math.floor(x_x / 10);
        x_x = new_x;
        reversed_num = reversed_num * 10 + last_digit;
        const current_num = Math.floor(x / Math.pow(10, i));

        if (current_num !== reversed_num) {
            return false;
        }

        if (x_x === 0) {
            loop = false;
        }

        

        i--;
    }
    if (x === reversed_num) {
        return true
    } else {
        return false;
    }
};

const result = isPalindrome(1212121)
console.log(result)