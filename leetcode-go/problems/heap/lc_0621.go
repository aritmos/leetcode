package heap

import "container/heap"

// Task Scheduler

type task struct {
	// no need to hold the "byte"
	count int
	wait  int
}

type TaskScheduler []task

// impl sort.Interface
func (ts TaskScheduler) Len() int {
	return len(ts)
}
func (ts TaskScheduler) Less(i, j int) bool {
	// mixed-heap by smallest wait then largest count
	return (ts[i].wait < ts[j].wait) ||
		((ts[i].wait == ts[j].wait) && (ts[i].count > ts[j].count))
}
func (ts TaskScheduler) Swap(i, j int) {
	ts[i], ts[j] = ts[j], ts[i]
}

// impl heap.Interface
func (ts *TaskScheduler) Push(x any) {
	*ts = append(*ts, x.(task))
}
func (ts *TaskScheduler) Pop() any {
	l := len(*ts)
	elem := (*ts)[l-1]
	*ts = (*ts)[:l-1]
	return elem
}

func NewTaskScheduler(counts []int) TaskScheduler {
	ts := TaskScheduler{}
	for _, count := range counts {
		if count != 0 {
			ts = append(ts, task{count: count})
		}
	}
	return ts
}

// reduce all non-zero wait times by one
func (ts *TaskScheduler) passTime() {
	for i := range *ts {
		if (*ts)[i].wait > 0 {
			(*ts)[i].wait--
		}
	}
}

func leastInterval(tasks []byte, n int) int {
	// max heap holding counts and waits
	// iterate through bytes greedy based on (max wait, max count)

	// accumulate byte slice into counts
	counts := [26]int{}
	for _, task := range tasks {
		counts[task-'A']++
	}
	ts := NewTaskScheduler(counts[:])
	heap.Init(&ts)

	time := 0
	for ; len(ts) > 0; time++ {
		task := heap.Pop(&ts).(task)
		if task.wait > 0 {
			heap.Push(&ts, task)
		} else {
			task.count--
			if task.count > 0 {
				task.wait = n + 1 // offset needed due to how we update waits
				heap.Push(&ts, task)
			}
		}
		ts.passTime()
		heap.Init(&ts)
	}

	return time
}

// smart mathematical solution skipping all simulation
// https://leetcode.com/problems/task-scheduler/solutions/4895014/c-go-find-max-frequency-o-n-solution/
func leastIntervalAlt(tasks []byte, n int) int {
	if n == 0 {
		return len(tasks)
	}

	counts := [26]int{}
	for _, task := range tasks {
		counts[task-'A']++
	}

	var maxCount, sameMaxCount int
	for _, c := range counts {
		if c > maxCount {
			maxCount = c
			sameMaxCount = 1
		} else if c == maxCount {
			sameMaxCount++
		}
	}

	res := (n+1)*(maxCount-1) + sameMaxCount
	if res > len(tasks) {
		return res
	} else {
		return len(tasks)
	}
}
