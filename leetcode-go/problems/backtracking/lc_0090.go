package backtracking

// Subsets II

import (
	"math"
	"slices"
)

func subsetsWithDup(nums []int) [][]int {
	out := [][]int{}
	powerSetSize := int(math.Pow(2.0, float64(len(nums))))
	for i := 0; i < powerSetSize; i++ {
		tmp := []int{}
		for j := 0; j < len(nums); j++ {
			if (1<<j)&i != 0 {
				tmp = append(tmp, nums[j])
			}
		}
		slices.Sort(tmp)
		var equalToTmp func([]int) bool = func(a []int) bool {
			return equalSlices(a, tmp)
		}
		if !slices.ContainsFunc(out, equalToTmp) {
			out = append(out, tmp)
		}
	}
	return out
}

func equalSlices(a, b []int) bool {
	if len(a) != len(b) {
		return false
	}
	for i, x := range a {
		if x != b[i] {
			return false
		}
	}
	return true
}
