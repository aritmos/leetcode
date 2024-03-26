package linkedlist

// Lowest Common Ancestor

func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	var inner func(*TreeNode, int, int) *TreeNode
	inner = func(root *TreeNode, x, y int) *TreeNode {
		if p.Val <= root.Val && root.Val <= q.Val {
			return root
		}
		if root.Val > y {
			return inner(root.Left, x, y)
		}
		if root.Val < x {
			return inner(root.Right, x, y)
		}
		panic("unreachable")
	}

	x, y := p.Val, q.Val
	if y > x { // technically allowed by the constraints
		x, y = y, x
	}
	return inner(root, x, y)
}
