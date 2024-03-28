package backtracking

import "slices"

// Palindrome Partitioning

func partition(s string) [][]string {
	out := [][]string{}
	stack := []string{}

	var dfsPartition func(idx int)
	dfsPartition = func(idx int) {
		rem := s[idx:]
		if len(rem) == 0 {
			out = append(out, slices.Clone(stack))
		}
		for i := 1; i <= len(rem); i++ {
			if isPalindrome(rem[:i]) {
				stack = append(stack, rem[:i])
				dfsPartition(idx + i)
				stack = stack[:len(stack)-1]
			}
		}
	}

	dfsPartition(0)
	return out
}

func isPalindrome(s string) bool {
	l, r := 0, len(s)-1
	for l < r {
		if s[l] != s[r] {
			return false
		}
		l++
		r--
	}
	return true
}
