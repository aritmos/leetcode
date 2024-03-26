package trees

// Serialise and Deserialise Binary Tree
// <BFS (leetcode style) for extra difficulty>

import (
	"fmt"
	"strconv"
	"strings"
)

type Codec struct {
}

func Constructor() Codec {
	return Codec{}
}

// Serializes a tree to a single string.
func (this *Codec) serialize(root *TreeNode) string {
	if root == nil {
		return "[]"
	}
	queue := []*TreeNode{root}
	seen := []string{}

	for len(queue) > 0 {
		node := queue[0]
		queue = queue[1:]

		if node == nil {
			seen = append(seen, "null")
			continue
		}

		seen = append(seen, fmt.Sprint(node.Val))
		queue = append(queue, node.Left)
		queue = append(queue, node.Right)
	}

	var lastNonNullIdx int
	for i := len(seen) - 1; i >= 0; i-- {
		if seen[i] != "null" {
			lastNonNullIdx = i
			break
		}
	}

	seen = seen[:lastNonNullIdx+1]

	return fmt.Sprintf("[%s]", strings.Join(seen, ","))
}

// Deserializes your encoded data to tree.
func (this *Codec) deserialize(data string) *TreeNode {
	values := this.parse(data)

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

// turns the `any` into a node with said value,
// if non-nil adds it to the slice, also returns the node
func numToNode(intOrNil any, slice []*TreeNode) *TreeNode {
	if intOrNil == nil {
		return nil
	}
	node := &TreeNode{Val: intOrNil.(int)}
	slice = append(slice, node)
	return node
}

func (this *Codec) parse(data string) []any {
	// strip brackets
	data = data[1 : len(data)-1]
	strNodes := strings.Split(data, ",")
	if len(strNodes) == 1 && strNodes[0] == "" {
		return []any{}
	}
	nodes := []any{}
	for _, sNode := range strNodes {
		if sNode == "null" {
			nodes = append(nodes, nil)
		} else {
			num, _ := strconv.Atoi(sNode)
			nodes = append(nodes, num)
		}
	}
	return nodes
}

/**
 * Your Codec object will be instantiated and called as such:
 * ser := Constructor();
 * deser := Constructor();
 * data := ser.serialize(root);
 * ans := deser.deserialize(data);
 */
