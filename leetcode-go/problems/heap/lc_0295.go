package heap

// Find Median from Data Stream

import "container/heap"

type maxHeap []int
type minHeap []int

// impl sort.Interface
func (mh maxHeap) Len() int           { return len(mh) }
func (mh minHeap) Len() int           { return len(mh) }
func (mh maxHeap) Less(i, j int) bool { return mh[i] > mh[j] }
func (mh minHeap) Less(i, j int) bool { return mh[i] < mh[j] }
func (mh maxHeap) Swap(i, j int)      { mh[i], mh[j] = mh[j], mh[i] }
func (mh minHeap) Swap(i, j int)      { mh[i], mh[j] = mh[j], mh[i] }

// impl heap.Interface
func (mh *maxHeap) Push(x any) { *mh = append(*mh, x.(int)) }
func (mh *minHeap) Push(x any) { *mh = append(*mh, x.(int)) }
func (mh *maxHeap) Pop() any {
	elem := (*mh)[len(*mh)-1]
	*mh = (*mh)[:len(*mh)-1]
	return elem
}
func (mh *minHeap) Pop() any {
	elem := (*mh)[len(*mh)-1]
	*mh = (*mh)[:len(*mh)-1]
	return elem
}

type MedianFinder struct {
	lhs maxHeap
	rhs minHeap
}

func Constructor() MedianFinder {
	mf := MedianFinder{}
	heap.Init(&mf.lhs)
	heap.Init(&mf.rhs)
	return mf
}

func (this *MedianFinder) AddNum(num int) {
	if len(this.lhs) > len(this.rhs) {
		if this.lhs[0] > num {
			tmp := heap.Pop(&this.lhs).(int)
			heap.Push(&this.rhs, tmp)
			heap.Push(&this.lhs, num)
		} else {
			heap.Push(&this.rhs, num)
		}
	} else {
		if len(this.rhs) > 0 && num > this.rhs[0] {
			tmp := heap.Pop(&this.rhs).(int)
			heap.Push(&this.lhs, tmp)
			heap.Push(&this.rhs, num)
		} else {
			heap.Push(&this.lhs, num)
		}
	}

}

func (this *MedianFinder) FindMedian() float64 {
	if len(this.lhs) > len(this.rhs) {
		return float64(this.lhs[0])
	} else {
		return float64(this.lhs[0]+this.rhs[0]) / 2.0
	}
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * obj := Constructor();
 * obj.AddNum(num);
 * param_2 := obj.FindMedian();
 */
