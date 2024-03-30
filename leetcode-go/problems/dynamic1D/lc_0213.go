package dynamic1d

// House Robber II

// essentially do house robber I from [..-1] and [1..] and take the max

func robI(nums []int) int {
	if len(nums) == 1 {
		return nums[0]
	}
	a1, b1, a2, b2 := 0, 0, 0, -nums[0]
	for _, n := range nums {
		a1, b1 = maxInt(b1+n, a1), a1
		a2, b2 = maxInt(b2+n, a2), a2
	}
	return maxInt(b1, a2)
}

func robII(nums []int) int {
	return maxInt(robI(nums[:len(nums)-1]), robI(nums[1:]))
}
