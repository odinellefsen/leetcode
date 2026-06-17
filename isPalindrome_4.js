// time complexity is O(n)
/**
 * @param {number} x
 * @return {boolean}
 */
var isPalindrome = function(x) {
    let shrinking_x = x;
    let growing_x = 0;

    while (shrinking_x > growing_x) {
        const lst_dgt_shr_x = shrinking_x % 10;
        shrinking_x = Math.floor(shrinking_x / 10);
        growing_x = growing_x * 10 + lst_dgt_shr_x;
    }
    
    if (growing_x === shrinking_x || Math.floor(growing_x / 10) === shrinking_x) {
        return true
    } else {
        return false;
    }
};

const result = isPalindrome(1212121);
console.log(result);