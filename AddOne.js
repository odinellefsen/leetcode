/**
 * @param {number[]} digits
 * @return {number[]}
 */
function plusOne(digits) {
	const combined = digits.join("");
	const numberfied = BigInt(combined) + 1n;
	const newCombinedValue = numberfied.toString();
	const newArray = newCombinedValue.split("");

	return newArray;
}

const output = plusOne([
	6, 1, 4, 5, 3, 9, 0, 1, 9, 5, 1, 8, 6, 7, 0, 5, 5, 4, 3,
]);

console.log("output: ", output);
