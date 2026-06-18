/**
 * @param {string} s
 * @return {number}
 */

const roman_to_value_mapping = {
    "I": 1,
    "V": 5,
    "X": 10,
    "L": 50,
    "C": 100,
    "D": 500,
    "M": 1000
}

var romanToInt = function(s) {
    let converted_value = 0;
    for (let i = 0; i < s.length; i++) {
        console.log("converted value: ", converted_value);
        const current_value = roman_to_value_mapping[s[i]];
        const former_value = roman_to_value_mapping[s[i-1]];
        
        if (i !== 0 && current_value < former_value) {
            converted_value += current_value;
        }
        else if (i !== 0 && current_value > former_value) {
            converted_value -= current_value;
        }
        else if (i === 0) {
            converted_value = current_value;
        }
    }

    return converted_value;
};

const result = romanToInt("LVIII");

console.log("Result: ", result);