function binarySearch(nums: number[], target: number): number {
	let left = 0;
	let right: number = nums.length - 1;

	while (left <= right) {
		const mid: number = Math.floor((left + right) / 2);
		const currentValue: number = nums[mid];

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

const result = binarySearch([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5);

console.log(result);
