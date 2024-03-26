package trees

// Kth Smallest Element in a BST

type nodeCount struct {
	count int
	value int
}

func (n *nodeCount) findkth(node *TreeNode, k int) {
	if node == nil {
		return
	}

	n.findkth(node.Left, k)

	n.count += 1
	if n.count == k {
		n.value = node.Val
		return
	}

	n.findkth(node.Right, k)
}

func kthSmallest(root *TreeNode, k int) int {
	n := &nodeCount{
		count: 0,
		value: -1,
	}

	n.findkth(root, k)

	return n.value
}
