package trees

import "testing"

func CreateTreeFromSlice(values []any) *TreeNode {
	if len(values) == 0 {
		return nil
	}

	// Create a queue to hold the nodes to be processed
	queue := []*TreeNode{}

	// Create the root node
	root := &TreeNode{Val: values[0].(int)}
	queue = append(queue, root)

	// Process the rest of the slice
	i := 1
	for len(queue) > 0 {
		node := queue[0]
		queue = queue[1:]

		// Process the left child
		if i < len(values) && values[i] != nil {
			node.Left = &TreeNode{Val: values[i].(int)}
			queue = append(queue, node.Left)
		}
		i++

		// Process the right child
		if i < len(values) && values[i] != nil {
			node.Right = &TreeNode{Val: values[i].(int)}
			queue = append(queue, node.Right)
		}
		i++
	}

	return root
}

func EqualSlices(a, b []int) bool {
	if len(a) != len(b) {
		return false
	}
	for i, v := range a {
		if v != b[i] {
			return false
		}
	}
	return true
}

func Equal2DSlices(a, b [][]int) bool {
	if len(a) != len(b) {
		return false
	}
	for i, v := range a {
		if !EqualSlices(v, b[i]) {
			return false
		}
	}
	return true
}

func Print2DSlice(slice [][]int) {
	print("[")
	for _, inner := range slice {
		print("[")
		for _, v := range inner {
			print(v, " ")
		}
		print("]")
	}
	print("]")
	println()
}

func TestTreeProblem(t *testing.T) {
	// root := CreateTreeFromSlice([]any{9, 6, -3, nil, nil, -6, 2, nil, nil, 2, nil, -6, -6, -6})
	codec := Constructor()
	// s := "[1,2,3,null,null,4,5]"
	// root := codec.deserialize(s)
	// ans := codec.serialize(root)
	// if s != ans {
	// 	t.Fatal(ans)
	// }

	s := "[]"
	root := codec.deserialize(s)
	ans := codec.serialize(root)
	if s != ans {
		t.Fatal(ans)
	}
}
