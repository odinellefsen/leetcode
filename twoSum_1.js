// slow because O(n^2)
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function(nums, target) {
    for (let i = 0; i < nums.length; i++) {
        const i_val = nums[i];
        for (let j = 0; j < nums.length; j++) {
            const j_val = nums[j];
            if (i === j) {
                continue;
            }

            const total = i_val + j_val;
            if (total === target) {
                return [i, j];
            }
        }
    }
};