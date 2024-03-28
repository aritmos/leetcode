package heap

// Last Stone Weight

import "container/heap"

type StoneHeap []int

// impl sort.Interface
func (sh StoneHeap) Len() int {
	return len(sh)
}
func (sh StoneHeap) Less(i, j int) bool {
	// max-heap
	return sh[i] > sh[j]
}
func (sh StoneHeap) Swap(i, j int) {
	sh[i], sh[j] = sh[j], sh[i]
}

// impl heap.Interface
func (sh *StoneHeap) Push(x any) {
	*sh = append(*sh, x.(int))
}
func (sh *StoneHeap) Pop() any {
	elem := (*sh)[len(*sh)-1]
	*sh = (*sh)[:len(*sh)-1]
	return elem
}

func lastStoneWeight(stones []int) int {
	var sh StoneHeap = stones
	heap.Init(&sh)
	for len(sh) > 1 {
		stone1, stone2 := heap.Pop(&sh).(int), heap.Pop(&sh).(int)
		if rem := stone1 - stone2; rem > 0 {
			heap.Push(&sh, rem)
		}
	}
	if len(sh) > 0 {
		return sh[0]
	}
	return 0
}
