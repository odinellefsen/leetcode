/**
 * @param {string} s
 * @return {boolean}
 */
var isValid = function(s) {
    const map = new Map();

    for (let i = 0; i < s.length; i++) {

        if (s[i] === "(") {
            map.set("(",  map.get("(") ?? 0 + 1);
        }
        if (s[i] === "[") {
            map.set("[", map.get("[") ?? 0 + 1);
        }
        if (s[i] === "{") {
            map.set("{", map.get("{") ?? 0 + 1);
        }
        
        if (s[i] === ")" && map.get("(") >= 1) {
            map.set("(", map.get("(") - 1);
        } else if (s[i] === ")") {
            return false;
        }
        if (s[i] === "]" && map.get("[") >= 1) {
            map.set("[", map.get("[") - 1);
        } else if (s[i] === "]") {
            return false;
        }
        if (s[i] === "}" && map.get("{") >= 1) {
            map.set("{", map.get("{") - 1);
        } else if (s[i] === "}") {
            return false;
        } 
    }

    for (const [key, value] of map) {
        if (value !== 0) {
            return false;
        }
    }

    return true;
};

const result = isValid("()[]{}");

console.log("result: ", result)