/**
 * @param {string} s
 * @return {boolean}
 */
var isValid = function(s) {
    const stack = [];

    for (const char of s) {
        console.log("char: ", char);
    }
};

const result = isValid("()[]{}");

console.log("result: ", result);