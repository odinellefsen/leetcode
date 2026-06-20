/**
 * @param {string} s
 * @return {boolean}
 */
var isValid = function(s) {
    const stack = [];

    for (const char of s) {
        if (char === "(" || char === "[" || char === "{") {
            stack.push(char);
        }

        if (char === ")") {
            if (stack[stack.length - 1] === "(") {
                stack.pop()
            } else {
                return false;
            }
        }
        if (char === "]") {
            if (stack[stack.length - 1] === "[") {
                stack.pop()
            } else {
                return false;
            }
        }
        if (char === "}") {
            if (stack[stack.length - 1] === "{") {
                stack.pop()
            } else {
                return false;
            }
        }
    }

    if (stack.length === 0) {
        return true;
    } else {
        return false;
    }
};

const result = isValid("()[]{}");

console.log("result: ", result);