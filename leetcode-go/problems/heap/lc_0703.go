package heap

import "container/heap"

type KthLargest struct {
	k    int
	vals []int
}

// impl sort.Interface
func (this KthLargest) Len() int {
	return len(this.vals)
}
func (this KthLargest) Less(i, j int) bool {
	return this.vals[i] < this.vals[j]
}
func (this KthLargest) Swap(i, j int) {
	this.vals[i], this.vals[j] = this.vals[j], this.vals[i]
}

// impl heap.Interface
func (this *KthLargest) Push(x any) {
	this.vals = append(this.vals, x.(int))
}
func (this *KthLargest) Pop() any {
	elem := this.vals[len(this.vals)-1]
	this.vals = this.vals[:len(this.vals)-1]
	return elem
}

// small optimisation over calling Add on every insertion
func Constructor_0703(k int, nums []int) KthLargest {
	// len(nums) >= k-1
	kl := KthLargest{k: k}
	if len(nums) >= k {
		kl.vals = nums[:k]
		heap.Init(&kl)
		for _, elem := range nums[k:] {
			kl.Add(elem)
		}
		return kl
	} else if len(nums) == k-1 {
		kl.vals = nums[:k-1]
		heap.Init(&kl)
		return kl
	} else {
		panic("not enough numbers to construct KthLargest struct")
	}
}

func (this *KthLargest) Add(val int) int {
	heap.Push(this, val)
	if len(this.vals) > this.k {
		heap.Pop(this)
	}
	return this.vals[0]
}
