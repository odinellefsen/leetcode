/**
 * @param {string[]} strs
 * @return {string}
 */
var longestCommonPrefix = function(strs) {
    let shortest_word = ""
    for (const current_word of strs) {
        if (current_word.length < shortest_word.length || shortest_word.length === 0) {
            shortest_word = current_word;
        }
    }
    
    let longest_common_prefix = "";
    outerLoop: for (let i = 0; i < shortest_word.length; i++) {
        let char_match = "";
        for (let j = 0; j < strs.length; j++) {
            if (strs[j][i] === char_match || char_match === "") {
                char_match = strs[j][i];
                continue;
            }

            break outerLoop;
        }
        longest_common_prefix += char_match;
    }
    return longest_common_prefix;
};

const result = longestCommonPrefix(["flower","flow","flight"])

console.log("result: ", result);