package backtracking

// Letter Combinations of a Phone Number

// 1:
// 2: abc
// 3: def
// 4: ghi
// 5: jkl
// 6: mno
// 7: pqrs
// 8: tuv
// 9: wxyz

var letters [][]byte = [][]byte{
	{},                   // 0
	{},                   // 1
	{'a', 'b', 'c'},      // 2
	{'d', 'e', 'f'},      // 3
	{'g', 'h', 'i'},      // 4
	{'j', 'k', 'l'},      // 5
	{'m', 'n', 'o'},      // 6
	{'p', 'q', 'r', 's'}, // 7
	{'t', 'u', 'v'},      // 8
	{'w', 'x', 'y', 'z'}, // 9
}

func letterCombinations(digits string) []string {
	out := []string{}
	stack := []byte{}
	digs := []int{}
	for _, b := range digits {
		digs = append(digs, int(b-'0'))
	}

	var backtrack func(idx int)
	backtrack = func(idx int) {
		rem := digs[idx:]
		if len(rem) == 0 {
			if len(stack) != 0 {
				out = append(out, string(stack))
			}
			return
		}
		for _, b := range letters[digs[idx]] {
			stack = append(stack, b)
			backtrack(idx + 1)
			stack = stack[:len(stack)-1]
		}
	}
	backtrack(0)
	return out
}
