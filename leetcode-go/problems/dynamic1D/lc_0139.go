package dynamic1d

// Word Break

import "strings"

func wordBreak(s string, wordDict []string) bool {
	dp := make([]bool, len(s)+1)
	dp[len(dp)-1] = true
	for i := len(s) - 1; i >= 0; i-- {
		for _, word := range wordDict {
			if strings.HasPrefix(s[i:], word) && dp[i+len(word)] {
				dp[i] = true
				break
			}
		}
	}
	return dp[0]
}
