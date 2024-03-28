package backtracking

import "slices"

// Permutations

func permute(nums []int) [][]int {
	var permuteInner func([]int, []int, *[][]int)
	permuteInner = func(curr, rem []int, out *[][]int) {
		if len(rem) == 0 {
			*out = append(*out, curr)
			return
		}

		for i, x := range rem {
			newCurr := append(slices.Clone(curr), x)
			newRem := slices.Clone(rem)
			if i == len(rem)-1 {
				newRem = newRem[:i]
			} else {
				newRem = append(newRem[:i], newRem[i+1:]...)
			}
			permuteInner(newCurr, newRem, out)
		}
	}

	out := [][]int{}
	permuteInner([]int{}, nums, &out)
	return out
}
