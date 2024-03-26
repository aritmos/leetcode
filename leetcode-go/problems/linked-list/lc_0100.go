package linkedlist

// Same Tree

func isSameTree(p *TreeNode, q *TreeNode) bool {
	if (p == nil) != (q == nil) {
		return false
	}
	if p == nil {
		return true
	}
	if p.Val != q.Val {
		return false
	}
	return isSameTree(p.Left, q.Left) && isSameTree(p.Right, q.Right)
}
