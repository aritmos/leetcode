package backtracking

// Subsets

import "math"

func subsets(nums []int) [][]int {
	subsetSize := int(math.Pow(2, float64(len(nums))))
	ret := make([][]int, 0, subsetSize)
	for i := range subsetSize {
		tmp := []int{}
		for j := range nums {
			if (1<<j)&i != 0 {
				tmp = append(tmp, nums[j])
			}
		}
		ret = append(ret, tmp)
	}
	return ret
}
