package dynamic1d

// Min Cost of Climbing Stairs

func minCostClimbingStairs(cost []int) int {
	a, b := cost[0], cost[1]
	for i := 2; i < len(cost); i++ {
		c := cost[i] + minInt(a, b)
		a, b = b, c
	}
	return minInt(a, b)
}

// faster than Go's generic `min()` implementation
func minInt(x, y int) int {
	if x <= y {
		return x
	} else {
		return y
	}
}
