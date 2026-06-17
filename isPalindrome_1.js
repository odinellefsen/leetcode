// time complexity is O(n)
/**
 * @param {number} x
 * @return {boolean}
 */
var isPalindrome = function(x) {
    let i = 0;
    const x_str = String(x);
    const half_length = Math.floor(x_str.length / 2);
    
    while (i < half_length) {
        const from_frst_half = x_str[i];
        const from_scnd_half = x_str[x_str.length - 1 - i];
        
        if (from_frst_half !== from_scnd_half) {
            return false;
        }

        i++;
    }

    return true;
};

const result = isPalindrome(1212121);
console.log(result);