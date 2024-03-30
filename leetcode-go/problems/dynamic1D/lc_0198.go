package dynamic1d

// House Robber

func rob(nums []int) int {
	a, b := 0, 0
	for _, n := range nums {
		a, b = maxInt(a+n, b), a
	}
	return a
}

func maxInt(x, y int) int {
	if x >= y {
		return x
	} else {
		return y
	}
}

//   [2, 7, 9, 3, 1]
// f:[12,10,10,3, 1]
// b:[             ]

//   [2, 1, 1, 2, 3]
// f:[6, 4, 4, 2, 3]

// last three are fixed,
// at each cost[i] = max(nums[i]+max(nums[i+2], nums[i+3]), nums[i+1])
