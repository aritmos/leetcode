package linkedlist

// Merge K Sorted Lists

// func allNil[T any](slice []*T) bool {
// 	for _, ptr := range slice {
// 		if ptr != nil {
// 			return false
// 		}
// 	}
// 	return true
// }

import "container/heap"

type orderedNodes []*ListNode

// impl sort.Interface
func (this orderedNodes) Len() int {
	return len(this)
}
func (this orderedNodes) Less(i, j int) bool {
	// inverted so we get a pop of min values (instead of max)
	return this[i].Val < this[j].Val
}
func (this orderedNodes) Swap(i, j int) {
	this[i], this[j] = this[j], this[i]
}

// impl heap.Interface
func (this *orderedNodes) Push(item any) {
	*this = append(*this, item.(*ListNode))
}
func (this *orderedNodes) Pop() any {
	old := *this
	n := old.Len()
	node := old[n-1]
	*this = old[0 : n-1]
	return node
}

func mergeKLists(lists []*ListNode) *ListNode {
	preHead := &ListNode{}
	curr := preHead

	// generate candidates of pointers to candidates
	candidates := make(orderedNodes, 0)
	for _, head := range lists {
		if head != nil {
			heap.Push(&candidates, head)
		}
	}
	heap.Init(&candidates)

	for len(candidates) != 0 {
		minNode := heap.Pop(&candidates).(*ListNode)
		curr.Next = minNode
		curr = minNode

		// re-add to list
		if next := minNode.Next; next != nil {
			heap.Push(&candidates, next)
		}
	}

	return preHead.Next
}
