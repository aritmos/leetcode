package dynamic1d

// Decode Ways

func numDecodings(s string) int {
	dp := make([]int, len(s)+1)
	dp[len(s)] = 1
	if s[len(s)-1] == '0' {
		dp[len(s)-1] = 0
	} else {
		dp[len(s)-1] = 1
	}
	for i := len(s) - 2; i >= 0; i-- {
		if s[i] != '0' {
			dp[i] += dp[i+1]
		}
		if s[i] == '1' || (s[i] == '2' && s[i+1] < '7') {
			dp[i] += dp[i+2]
		}
	}
	return dp[0]
}

// [6 0 1 1]
//  1 0 6

// [12]<3>
// [1][2]<3>
// [12]<3>

// [121] => [12]<1>
//          [1][2]<1>
//           [12]<1>
// [21] = <2>
//
// s [1  2  1]
// c [3  2  1] 1
