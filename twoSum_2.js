/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function(nums, target) {
    const map = new Map();
    for (let i = 0; i < nums.length; i++) {
        const current_value = nums[i];
        console.log("current_value: ", current_value);
        const difference = target - current_value;
        console.log("difference: ", difference);
        const keyExists = map.has(current_value);
        
        if (keyExists && current_value * 2 === target) {
            console.log("Oof")
            const index = map.get(current_value)
            return [i, index]
        }

        const diff_get = map.get(difference)
        console.log("diff_get: ", diff_get)

        if (diff_get !== undefined) {
            return [i, diff_get]
        }

        map.set(current_value, i)
    }
    
};

const result = twoSum([2,7,11,15], 9);

console.log(result);