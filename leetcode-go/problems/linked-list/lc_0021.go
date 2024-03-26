package linkedlist

// Merge Two Sorted Lists

func mergeTwoLists(node1 *ListNode, node2 *ListNode) *ListNode {
	head := &ListNode{}
	current := head

	for node1 != nil && node2 != nil {
		if node1.Val <= node2.Val {
			current.Next = node1
			node1 = node1.Next
		} else {
			current.Next = node2
			node2 = node2.Next
		}
		current = current.Next
	}

	if node1 != nil {
		current.Next = node1
	} else {
		current.Next = node2
	}

	return head.Next
}
