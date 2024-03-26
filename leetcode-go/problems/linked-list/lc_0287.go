package linkedlist

// Find the Duplicate Number

// // bad O(~n^2) approach with many cycles
// func findDuplicate(nums []int) int {
// 	for _, x := range nums {
// 		first_idx := x - 1
// 		slow_idx := first_idx
// 		fast_idx := nums[first_idx] - 1
//
// 		for slow_idx != fast_idx {
// 			slow_idx = nums[slow_idx] - 1
// 			fast_idx = nums[nums[fast_idx]-1] - 1
// 		}
//
// 		if nums[slow_idx]-1 == first_idx {
// 			return x
// 		}
// 	}
//
// 	panic("unreachable")
// }

// correct logic:
// the duplicate number causes a loop of this form:
//
// o -> o -> 0 -> o -> o
//                |    |
//                0 <- o
//
// the two `0`s are the duplicate numbers which then point to the same index
// this always happens when not offsetting the indices as we were doing before
//
// we then want to find the number at `0`. in order to do this we use the slow-fast
// approach. let the lengths of the hair and loop be `h` and `l` respectively
// (in the diagram we have the `h = 3`, `l = 4` case)
//
// starting at the end of the hair--when the slow and fast pointers meet after `n` jumps--
// we know that:
// > `n` is the smallest int s.t. `(2n - h) = (n - h) modulo l`
// this must always exist by the closed nature of the loop (use the LCM of the nums)
// > `2n = n modulo l`
// > `n = L` (as `n` is the smallest number with said property above)
//
// > fast is at position `(2n - h) % l = -h` in the loop
// > restarting slow at the origin means if the fast pointer now does jumps of 1,
//   that they will both meet at the start of the loop:
//   Critically this means right before they meet they are both one away, with matching
//   pointers. It is in this state that both pointers contain the duplicate number

func findDuplicate(nums []int) int {
	slow := nums[0]
	fast := nums[0]

	for {
		slow = nums[slow]
		fast = nums[nums[fast]]

		if slow == fast {
			break
		}
	}

	slow = nums[0]
	for slow != fast {
		slow = nums[slow]
		fast = nums[fast]
	}

	return slow
}
