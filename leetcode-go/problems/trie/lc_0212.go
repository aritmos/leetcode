package trie

// Word Search II

type ModdedTrie struct {
	children      [26]*ModdedTrie
	offsetWordIdx int // offset by one to remove branching
}

func (this *ModdedTrie) Insert(word string, offsetIdx int) {
	node := this
	for _, char := range word {
		idx := char - 'a'
		if node.children[idx] == nil {
			node.children[idx] = &ModdedTrie{}
		}
		node = node.children[idx]
	}
	node.offsetWordIdx = offsetIdx
}

func findWords(board [][]byte, words []string) []string {
	trie := ModdedTrie{}
	for i, word := range words {
		trie.Insert(word, i+1) // offset the index by 1!
	}

	seen := make([]bool, len(words)+1) // include space for offset
	rows, cols := len(board), len(board[0])

	for y := 0; y < rows; y++ {
		for x := 0; x < cols; x++ {
			backtrackSearch(board, &trie, x, y, []byte{}, &seen)
		}
	}

	out := []string{}
	for i, b := range seen[1:] {
		if b {
			out = append(out, words[i])
		}
	}
	return out
}

func backtrackSearch(board [][]byte, trie *ModdedTrie, x, y int, curr []byte, seen *[]bool) {
	if x < 0 || x >= len(board[0]) || y < 0 || y >= len(board) || board[y][x] == 0 {
		return // Out of bounds or already visited
	}

	char := board[y][x]
	node := trie.children[char-'a']
	if node == nil {
		return // No further path in trie
	}

	board[y][x] = 0
	curr = append(curr, char)

	// if the node corresponds to a whole word, add it to seen
	(*seen)[node.offsetWordIdx] = true

	// Explore all possible directions
	deltas := [4][2]int{{1, 0}, {0, -1}, {-1, 0}, {0, 1}}
	for _, delta := range deltas {
		newX, newY := x+delta[0], y+delta[1]
		backtrackSearch(board, node, newX, newY, curr, seen)
	}

	board[y][x] = char
}
