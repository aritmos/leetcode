package trees

func levelOrder(root *TreeNode) [][]int {
	out := make([][]int, 0)
	nodes := []*TreeNode{root}
	children := []*TreeNode{}
	for i := 0; len(nodes) != 0; i++ {
		out = append(out, []int{})
		for _, node := range nodes {
			out[i] = append(out[i], node.Val)
			if node.Left != nil {
				children = append(children, node.Left)
			}
			if node.Right != nil {
				children = append(children, node.Right)
			}
		}
		nodes, children = children, nodes[:0]
	}
	return out
}
