class ListNode {
	constructor(val, next) {
		this.val = val === undefined ? 0 : val;
		this.next = next === undefined ? null : next;
	}
}

function reverseList(head) {
	let prev = null;
	let current = head;

	while (current !== null) {
		const next = current.next; // Store next node
		current.next = prev; // Reverse the link
		prev = current; // Move prev forward
		current = next; // Move current forward
	}

	return prev;
}

const node5 = new ListNode(5);
const node4 = new ListNode(4, node5);
const node3 = new ListNode(3, node4);
const node2 = new ListNode(2, node3);
const head = new ListNode(1, node2);

let current = head;
console.log("Original list:");
while (current !== null) {
	console.log(current.val);
	current = current.next;
}

const reversedHead = reverseList(head);

console.log("\nReversed list:");
current = reversedHead;
while (current !== null) {
	console.log(current.val);
	current = current.next;
}
