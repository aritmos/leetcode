package linkedlist

// Copy List With Random Pointer

func copyRandomList(head *Node) *Node {
	ptr_map := make(map[*Node]*Node)
	ptr_map[nil] = nil

	// the start of the new list
	new_head := &Node{}
	// the last node in the new list
	curr := new_head

	for head != nil {
		var node *Node

		if n, seen := ptr_map[head]; seen {
			node = n
			node.Val = head.Val
		} else {
			node = &Node{Val: head.Val}
			ptr_map[head] = node
		}

		// the `Next` field will be set when we traverse this node naturally

		// set the `Random` field
		if rand, seen := ptr_map[head.Random]; seen {
			node.Random = rand
		} else {
			// not seen: create it, add it to map, and set it within node
			rand = &Node{}
			ptr_map[head.Random] = rand
			node.Random = rand
		}

		// this sets the `Next` field of the node in the previous iteration
		curr.Next = node

		head = head.Next
		curr = curr.Next
	}

	return new_head.Next
}
