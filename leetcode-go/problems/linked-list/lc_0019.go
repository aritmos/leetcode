package linkedlist

// Remove Nth Node From End of List

func removeNthFromEnd(head *ListNode, n int) *ListNode {
	back := head
	front := head
	for i := 0; i < n; i++ {
		front = front.Next
	}

	// case: remove first node
	if front == nil {
		return back.Next
	}

	front = front.Next
	for front != nil {
		back = back.Next
		front = front.Next
	}

	// `back` is one node before the node that needs deleting
	back.Next = back.Next.Next

	return head
}
