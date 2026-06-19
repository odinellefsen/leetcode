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
    
    for (let i = 0; i < shortest_word.length; i++) {
        
    }

};

const result = longestCommonPrefix(["flower","flow","flight"])

// console.log("result: ", result);