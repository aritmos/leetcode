package linkedlist

// Reverse Linked List

func reverseList(node *ListNode) *ListNode {
	var prev *ListNode = nil

	for node != nil {
		next := node.Next
		node.Next = prev
		prev = node
		node = next
	}

	return prev
}
