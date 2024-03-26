package trees

// Binary Tree Maximum Path Sum

func maxPathSum(root *TreeNode) int {
	var dfs func(*TreeNode, *int) int
	dfs = func(node *TreeNode, maxPath *int) (maxBranch int) {
		if node == nil {
			return 0
		}

		leftMax := dfs(node.Left, maxPath)
		rightMax := dfs(node.Right, maxPath)
		maxSubBranch := max(leftMax, rightMax, 0)
		maxBranch = maxSubBranch + node.Val

		// each node checks its own largest
		// - (branch + self)
		*maxPath = max(*maxPath, maxBranch)
		// - (bridge)
		*maxPath = max(*maxPath, node.Val+leftMax+rightMax)

		return
	}

	maxPath := -2147483648
	dfs(root, &maxPath)

	return maxPath
}

// DFS:
// - keep track of max branch sum
// - calculate max path at each node
