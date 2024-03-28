package backtracking

// Word Search

func exist(board [][]byte, word string) bool {
	found := false
	for y := range len(board) {
		for x := range len(board[0]) {
			dfs(board, x, y, []byte(word), &found)
		}
	}
	return found
}

func dfs(board [][]byte, x, y int, word []byte, found *bool) {
	if *found || x < 0 || x >= len(board[0]) || y < 0 || y >= len(board) {
		return
	}
	char := board[y][x]
	if char == 0 || char != word[0] {
		return
	}
	if len(word) == 1 {
		// word found!
		*found = true
		return
	}
	board[y][x] = 0 // mark square as seen

	deltas := []int{1, 0, -1, 0, 1}
	for i := 0; i < len(deltas)-1; i++ {
		dx, dy := deltas[i], deltas[i+1]
		dfs(board, x+dx, y+dy, word[1:], found)
	}

	board[y][x] = char
}
