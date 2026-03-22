const nums = [
	3, 5, 7, 12, 13, 15, 18, 19, 20, 23, 27, 28, 33, 40, 43, 50, 51, 52, 53,
];

function BinarySearch(nums, target) {
	let left = 0;
	let right = nums.length - 1;

	while (left <= right) {
		const mid = Math.floor((left + right) / 2);
		const currentValue = nums[mid];

		if (currentValue === target) {
			return mid;
		}
		if (currentValue < target) {
			left = mid + 1;
		}
		if (currentValue > target) {
			right = mid - 1;
		}
	}
	return -1;
}

const output = BinarySearch(nums, 53);

console.log("index of target: ", output);
