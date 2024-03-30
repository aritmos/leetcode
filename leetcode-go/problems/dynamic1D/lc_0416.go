package dynamic1d

// Partition Equal Set Sum

type set []bool

func NewSet(length int) set {
	return make([]bool, length)
}

func (this *set) add(x int) {
	(*this)[x] = true
}

func (this set) contains(x int) bool {
	return this[x]
}

func canPartition(nums []int) bool {
	totalSum := 0
	for _, n := range nums {
		totalSum += n
	}
	if totalSum%2 != 0 {
		return false
	}
	target := totalSum / 2
	for _, num := range nums {
		if num > target {
			return false
		}
	}
	sums := NewSet(target + 1)
	sums.add(0)
	for _, n := range nums {
		if sums.contains(target) {
			return true
		}
		newSums := []int{} // separate as to not add to the loop below
		for s, exists := range sums {
			if exists && n+s <= target {
				newSums = append(newSums, n+s)
			}
		}
		for _, sum := range newSums {
			sums.add(sum)
		}
	}
	return sums.contains(target)
}
