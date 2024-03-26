package trees

// Binary Tree Right Side View

func rightSideView(root *TreeNode) []int {
	out := []int{}
	maxDepth := -1

	// right-to-left dfs
	var dfs func(node *TreeNode, depth int, out *[]int)
	dfs = func(node *TreeNode, depth int, out *[]int) {
		if node == nil {
			return
		}
		if depth > maxDepth {
			*out = append(*out, node.Val)
		}
		maxDepth++
		depth++
		dfs(node.Right, depth, out)
		dfs(node.Left, depth, out)
	}

	dfs(root, 0, &out)

	return out
}

// need:
// - right traversal of the tree (DFS)
//   - with depth state
