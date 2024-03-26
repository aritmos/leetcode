package trees

// Construct Binary Tree from Preorder and Inorder Traversal

import "slices"

func buildTree(preorder []int, inorder []int) *TreeNode {
	if len(preorder) == 0 {
		return nil
	}
	val := preorder[0]
	preorder = preorder[1:]
	node := &TreeNode{Val: val}

	// leaf case
	if len(preorder) == 0 {
		return node
	}

	idx := slices.Index(inorder, val)
	node.Left = buildTree(preorder[:idx], inorder[:idx])
	node.Right = buildTree(preorder[idx:], inorder[idx+1:])

	return node
}

// preorder: node -> left -> right
// inorder: left -> node -> right
