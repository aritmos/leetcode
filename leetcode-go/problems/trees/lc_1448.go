package trees

// Count Good Nodes in Binary Tree

func goodNodes(root *TreeNode) int {
	out := 0

	var dfs func(*TreeNode, int, *int)
	dfs = func(node *TreeNode, maxVal int, out *int) {
		if node == nil {
			return
		}
		if node.Val >= maxVal {
			maxVal = node.Val
			*out++
		}
		dfs(node.Left, maxVal, out)
		dfs(node.Right, maxVal, out)
	}

	dfs(root, -100000, &out)

	return out
}
