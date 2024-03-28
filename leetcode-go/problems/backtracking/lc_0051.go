package backtracking

// N-Queens

// 1 <= n <= 9
func solveNQueens(n int) [][]string {
	status := make([]int, n*n)
	stack := []int{}
	result := [][]string{}

	var backtrack func(int)
	backtrack = func(startIdx int) {
		// first queen must be in the first row
		// exclude searching when this isn't the case
		if len(stack) > 1 && stack[0] >= n {
			return
		}
		if len(stack) == n {
			result = append(result, stackToBoard(n, stack))
			return
		}

		for i := startIdx; i < n*n; i++ {
			if status[i] == 0 {
				stack = append(stack, i)
				modify(n, &status, i, 1)
				backtrack(i)
				modify(n, &status, i, -1)
				stack = stack[:len(stack)-1]
			}
		}
	}

	backtrack(0)
	return result
}

func stackToBoard(n int, stack []int) []string {
	board := make([]string, n)
	for i := range n {
		for range n {
			board[i] += "."
		}
	}
	for _, idx := range stack {
		row, col := idx/n, idx%n
		tmp := []byte(board[row])
		tmp[col] = 'Q'
		board[row] = string(tmp)
	}
	return board
}

func modify(n int, status *[]int, idx int, val int) {
	row, col := idx/n, idx%n
	l, r := col-row, -n+1+col+row

	for i := range n {
		(*status)[row*n+i] += val // horizontal
		(*status)[i*n+col] += val // vertical
	}
	// L->R downwards
	for x, y := max(l, 0), max(-l, 0); x < n && y < n; {
		(*status)[y*n+x] += val
		x++
		y++
	}
	// R->L downwards
	for x, y := n-1+min(r, 0), max(r, 0); x >= 0 && y < n; {
		(*status)[y*n+x] += val
		x--
		y++
	}
	// queen's position
	(*status)[row*n+col] -= 3 * val
}
