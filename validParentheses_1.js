/**
 * @param {string} s
 * @return {boolean}
 */
var isValid = function(s) {
    const map = new Map();

    for (let i = 0; i < s.length; i++) {

        if (s[i] === "(") {
            map.set("(",  map.get("(") ?? 0 + 1)
        }

        if (s[i] === "[") {
            map.set("[", map.get("[") ?? 0 + 1)
        }
        
        if (s[i] === ")" && map.get("(") >= 1) {
            map["("] = map.get("(") - 1;
        } else {
            return false;
        }
        if (s[i] === "]" && map.get("[") >= 1) {
            map["["] = map.get("[") - 1
        } else {
            return false;
        }


    }
};

