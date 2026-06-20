/**
 * @param {string} s
 * @return {boolean}
 */
var isValid = function(s) {
    const stack = [];

    for (const char of s) {
        console.log("stack: ", stack);
        if (char === "(" || char === "[" || char === "{") {
            stack.push(char);
        }

        if (char === ")") {
            if (char[s.length - 1] === "(") {
                stack.pop()
            } else {
                return false;
            }
        }
        if (char === "]") {
            if (char[s.length - 1] === "[") {
                stack.pop()
            } else {
                return false;
            }
        }
        if (char === "}") {
            if (char[s.length - 1] === "{") {
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