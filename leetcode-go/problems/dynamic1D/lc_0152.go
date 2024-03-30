package dynamic1d

// Maximum Product Subarray

func maxProduct(nums []int) int {
	result, maximum, minimum := nums[0], nums[0], nums[0]
	for _, n := range nums[1:] {
		if n < 0 {
			maximum, minimum = minimum, maximum
		}
		maximum = max(n, maximum*n)
		minimum = min(n, minimum*n)
		result = max(result, maximum)
	}
	return result
}

// [2 3 -2  4]
//M[2 6 -2  4
//m[0 0 -12 -48

// [2 2 0 3]
//f[2 4 0 3]
//b[4 2 0 3]
//t[2 4 0 0] => 4

// [2 3 -2 -2]
//M[2 6 -2  24
//m[0 0 -12 -2

//        v
// [2    1 -2   3 -1 8]
// [96  48 48   3 -1 8]
// [-12 -6 -6 -24 -8 8]

// trivial O(n^2) alg (brute force)

// zeros make boundaries,
// unless it is isolated negatives [-1 0 -1] in which case the max is 0
//
