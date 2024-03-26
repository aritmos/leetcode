package linkedlist

// Subtree of Another Tree

// func isSameTree(p *TreeNode, q *TreeNode) bool {
// 	return ((p == nil) == (q == nil)) &&
// 		((p == nil) || ((p.Val == q.Val) &&
// 			(isSameTree(p.Left, q.Left) && isSameTree(p.Right, q.Right))))
// }

func isSubtree(root *TreeNode, subRoot *TreeNode) bool {
	return isSameTree(root, subRoot) || ((root != nil) &&
		(isSubtree(root.Left, subRoot) || isSubtree(root.Right, subRoot)))
}
