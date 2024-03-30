package dynamic1d

// Longest Palindromic Substring

func longestPalindrome(s string) string {
	sLen := len(s)
	findPalindrome := func(s string, l, r int) (length int, out string) {
		for l >= 0 && r < sLen && s[l] == s[r] {
			l--
			r++
		}
		return r - l - 1, s[l+1 : r]
	}
	maxLen, out := 0, ""
	for i := 0; i < sLen; i++ {
		l1, o1 := findPalindrome(s, i, i)
		l2, o2 := findPalindrome(s, i, i+1)
		if l1 > maxLen {
			maxLen, out = l1, o1
		}
		if l2 > maxLen {
			maxLen, out = l2, o2
		}
	}
	return out
}
