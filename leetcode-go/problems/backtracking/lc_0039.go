package backtracking

// Combination Sum

func combinationSum(candidates []int, target int) [][]int {
	result := [][]int{}
	counts := make([]int, len(candidates))
	i, currSum := 0, 0
	for {
		// backtrack
		if i == len(candidates) {
			i--
			for ; i >= 0 && counts[i] == 0; i-- {
			}
			if i < 0 {
				break
			}

			counts[i]--
			currSum -= candidates[i]
			i++
			continue
		}

		currSum += candidates[i]
		counts[i]++

		remaining := target - currSum
		if remaining == 0 {
			nums := []int{}
			for i, c := range counts {
				for range c {
					nums = append(nums, candidates[i])
				}
			}
			result = append(result, nums)
		}
		if remaining < 0 {
			// roll back the last sum
			currSum -= candidates[i]
			counts[i]--

			// move the pointer
			i++
		}
	}
	return result
}
