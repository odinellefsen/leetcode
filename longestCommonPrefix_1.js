/**
 * @param {string[]} strs
 * @return {string}
 */
var longestCommonPrefix = function(strs) {
    for (const value of strs) {
        console.log(value)
    }

    console.log(strs[0] > strs[1]);

    let shortest_word = ""
    for (const current_word of strs) {
        if (current_word < shortest_word || shortest_word.length === 0) {
            console.log("new shortest word: ", current_word)
            shortest_word = current_word;
        }
    }
    console.log("shortest word:", shortest_word)
};

const result = longestCommonPrefix(["flower","flow","flight"])

// console.log("result: ", result);