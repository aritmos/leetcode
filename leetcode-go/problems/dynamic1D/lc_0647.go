package dynamic1d

// Palindromic Substrings

func countSubstrings(s string) int {
	sLen := len(s)
	countPalindromesFrom := func(s string, l, r int, count *int) {
		for l >= 0 && r < sLen && s[l] == s[r] {
			*count++
			l--
			r++
		}
	}
	count := 0
	for i := 0; i < sLen; i++ {
		countPalindromesFrom(s, i, i, &count)
		countPalindromesFrom(s, i, i+1, &count)
	}
	return count
}
