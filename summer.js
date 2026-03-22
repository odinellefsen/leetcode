const fs = require("node:fs");

const fileContent = fs.readFileSync("./sum_this.txt", "utf8");

const linesArray = fileContent.split("\n");
const numLinesArray = [];

for (let i = 0; i < linesArray.length; i++) {
	numLinesArray.push(Number(linesArray[i]));
}

const sum = numLinesArray.reduce((a, b) => a + b, 0);

console.log("sum: ", sum);
