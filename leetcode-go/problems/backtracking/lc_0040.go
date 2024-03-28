package backtracking

// Combination Sum II

import (
	"slices"
)

func combinationSum2(nums []int, target int) [][]int {
	slices.Sort(nums)

	// (optional) truncate to only keep numbers smaller than target
	idx, exists := slices.BinarySearch(nums, target)
	if exists {
		idx++
	}
	nums = nums[:idx]

	// turn list into non-duplicates and counts
	unique := []int{nums[0]}
	maxCounts := []int{1}
	for i := 1; i < len(nums); i++ {
		lastIndex := len(unique) - 1
		if nums[i] == unique[lastIndex] {
			// Increment count of the last unique number found
			maxCounts[lastIndex]++
		} else {
			// Append new unique number and its count
			unique = append(unique, nums[i])
			maxCounts = append(maxCounts, 1)
		}
	}

	// do the same as `Combination Sum` but reversed and with a hard max on the counts

	result := [][]int{}
	counts := make([]int, len(unique))
	i, currSum := len(unique)-1, 0
	for {
		// backtrack
		if i == -1 {
			i++
			// move to the first non-zero count element
			for ; i < len(unique) && counts[i] == 0; i++ {
			}
			if i == len(unique) {
				break
			}

			counts[i]--
			currSum -= unique[i]
			i--
			continue
		}
		if currSum >= target {
			currSum -= unique[i]
			counts[i]--
			i--
			continue
		}

		if counts[i] == maxCounts[i] {
			// cant do anything, need to move forward
			i--
			continue
		}

		counts[i]++
		currSum += unique[i]
		remaining := target - currSum

		if remaining == 0 {
			nums := []int{}
			for i, c := range counts {
				for range c {
					nums = append(nums, unique[i])
				}
			}
			result = append(result, nums)
		} else if remaining < 0 {
			// roll back the last sum
			currSum -= unique[i]
			counts[i]--

			// move the pointer
			i--
		}
	}
	return result
}
