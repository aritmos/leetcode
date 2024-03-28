package heap

// Kth Largest Element in Array

import "container/heap"

type MaxHeap []int

// impl sort.Interface
func (mh MaxHeap) Len() int {
	return len(mh)
}
func (mh MaxHeap) Less(i, j int) bool {
	// min-heap
	return mh[i] < mh[j]
}
func (mh MaxHeap) Swap(i, j int) {
	mh[i], mh[j] = mh[j], mh[i]
}

// impl heap.Interface
func (mh *MaxHeap) Push(x any) {
	*mh = append(*mh, x.(int))
}
func (mh *MaxHeap) Pop() any {
	elem := (*mh)[len(*mh)-1]
	*mh = (*mh)[:len(*mh)-1]
	return elem
}

func findKthLargest(nums []int, k int) int {
	var mh MaxHeap = nums[:k]
	heap.Init(&mh)
	for _, n := range nums[k:] {
		heap.Push(&mh, n)
		heap.Pop(&mh)
	}
	return mh[0]
}
