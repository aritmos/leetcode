package linkedlist

// Reverse Nodes in k-Group

func reverseKGroup(head *ListNode, k int) *ListNode {
	preHead := &ListNode{}
	newLast := preHead
	remainder := head
	i := 1
outer:
	for {
		head := remainder
		for node := remainder; node != nil; i++ {
			if i == k {
				remainder = node.Next
				newHead, newTail := reverseSubList(head, node)
				newLast.Next = newHead
				newLast = newTail
				i = 1
				continue outer
			} else {
				node = node.Next
			}
		}
		if i != 1 {
			newLast.Next = remainder
		}
		break
	}

	return preHead.Next
}

// reverses the list up to and including the `last` node
func reverseSubList(head, last *ListNode) (newHead, newLast *ListNode) {
	var prev *ListNode = nil
	node := head
	for {
		next := node.Next // is always ok because list is valid upto `last`
		node.Next = prev
		prev = node

		if node == last {
			break
		}

		node = next
	}

	return prev, head
}
