class LinkedNode {
	constructor(value) {
		this.value = value;
		this.next = undefined;
	}
}

class LinkedList {
	constructor() {
		this.head = undefined;
		this.length = 0;
		this.current = undefined;
	}

	insert(value) {
		const newNode = new LinkedNode(value);
		this.length++;

		if (!this.head) {
			this.head = newNode;
			return;
		}

		let current = this.head;
		while (current.next) {
			current = current.next;
		}
		current.next = newNode;
	}

	goNext() {
		if (!this.current) {
			this.current = this.head;
		} else if (this.current.next) {
			this.current = this.current.next;
		} else {
			return undefined;
		}
		return this.current ? this.current.value : undefined;
	}
}

const list1 = new LinkedList();
const list2 = new LinkedList();

list1.insert(1);
list1.insert(3);
list1.insert(5);
list1.insert(7);
list1.insert(9);

list2.insert(2);
list2.insert(4);
list2.insert(6);
list2.insert(8);

let logList1 = list1.head;
while (logList1) {
	console.log("list1: ", logList1.value);
	logList1 = logList1.next;
}

console.log();

let logList2 = list2.head;
while (logList2) {
	console.log("list2: ", logList2.value);
	logList2 = logList2.next;
}

// combining the sorted lists to one combined sorted list.
let listOne = list1.head;
let listTwo = list2.head;

const finalList = new LinkedList();

while (listOne !== undefined && listTwo !== undefined) {
	if (listOne.value < listTwo.value) {
		finalList.insert(listOne.value);
		listOne = listOne.next;
	} else {
		finalList.insert(listTwo.value);
		listTwo = listTwo.next;
	}
}

while (listOne !== undefined) {
	finalList.insert(listOne.value);
	listOne = listOne.next;
}

while (listTwo !== undefined) {
	finalList.insert(listTwo.value);
	listTwo = listTwo.next;
}

let finalListLog = finalList.head;
while (finalListLog) {
	console.log("sorted list:", finalListLog.value);
	finalListLog = finalListLog.next;
}
