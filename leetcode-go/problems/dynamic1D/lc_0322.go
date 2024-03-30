package dynamic1d

// Coin Change

func coinChange(coins []int, amount int) int {
	counts := make([]int, amount+1)
	intMax := int(^uint(0) >> 1) // max int
	for i := 1; i < len(counts); i++ {
		numCoins := intMax
		for _, coin := range coins {
			prevIdx := i - coin
			if prevIdx == 0 && numCoins > 1 {
				numCoins = 1
			} else if prevIdx > 0 {
				count := counts[prevIdx] + 1
				if count > 1 && count < numCoins {
					numCoins = count
				}
			}
		}
		if numCoins != intMax {
			counts[i] = numCoins
		}
	}
	ans := counts[len(counts)-1]
	if ans == 0 && amount != 0 {
		return -1
	}
	return ans
}

// if a coin is selected the answer becomes 1 + dp[amount-coint]
// coins = [2], amount = 3
// [0 1 2 3]
// [0 0 1 0]

// [1,2,5,10]
// [0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27]
// [0  1  1  2  2  1  1  ]
