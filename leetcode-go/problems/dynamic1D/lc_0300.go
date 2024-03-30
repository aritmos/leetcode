package dynamic1d

import "slices"

// Longest Increasing Subsequence

func lengthOfLIS(nums []int) int {
	dp := make([]int, len(nums))
	for i := 0; i < len(nums); i++ {
		prevCount := 0
		for j := i - 1; j >= prevCount; j-- {
			if nums[j] < nums[i] {
				prevCount = maxInt(prevCount, dp[j])
			}
		}
		dp[i] = prevCount + 1
	}
	return slices.Max(dp)
}

// [1,3,6,7,9,4,10,5,6]
//s[1 3 4 5 6 10

// func lengthOfLISb(nums []int) int {
// 	stack := []int{}
//
// 	for _, num := range nums {
// 		if len(stack) == 0 || stack[len(stack)-1] < num {
// 			stack = append(stack, num)
// 		} else {
// 			idx := findNextMaxNum(stack, num)
// 			stack[idx] = num
// 		}
// 	}
//
// 	return len(stack)
// }
//
// func findNextMaxNum(arr []int, num int) int {}

// [10,9,2,5,3,7,101,18] => [2,3,7,101] = 4
// [0,1,0,3,2,3] => 4
// [7,7,7,7,7,7] => 1

// 10 9 2 5 3 7 101 18

// 5 1 6 2 7 3 8 4 9 5 6 7 8 9

// 1 1 2 2 3 3 4 4 5 5 6 7 8 9

// longest subsequence ending at index `i` is equal to the numbers n in nums[:i] with n < nums[i]
// longest LIS at `i` == (LIS of first n < nums[i], + 1)
