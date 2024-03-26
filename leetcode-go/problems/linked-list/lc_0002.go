package linkedlist

// Add Two Numbers

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	head := &ListNode{}
	curr := head
	var carry int

	for l1 != nil || l2 != nil {
		var v1 int
		var v2 int

		if l1 != nil {
			v1 = l1.Val
		} else {
			v1 = 0
		}

		if l2 != nil {
			v2 = l2.Val
		} else {
			v2 = 0
		}

		sum := v1 + v2 + carry
		digit := sum % 10
		carry = sum / 10

		curr.Next = &ListNode{Val: digit}
		curr = curr.Next

		if l1 != nil {
			l1 = l1.Next
		}
		if l2 != nil {
			l2 = l2.Next
		}
	}

	if carry == 1 {
		curr.Next = &ListNode{Val: 1}
	}

	return head.Next
}
