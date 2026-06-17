/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function(nums, target) {
    const map = new Map();
    for (let i = 0; i < nums.length; i++) {
        const current_value = nums[i];
        const difference = target - current_value;

        const diff_get = map.get(difference)

        if (diff_get !== undefined) {
            return [i, diff_get]
        }

        map.set(current_value, i)
    }
    
};

const result = twoSum([2,7,11,15], 9);

console.log(result);