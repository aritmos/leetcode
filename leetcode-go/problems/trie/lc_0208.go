package trie

type Trie struct {
	children       [26]*Trie
	isCompleteWord bool
}

func Constructor() Trie {
	return Trie{}
}

func (this *Trie) Insert(word string) {
	node := this
	for _, char := range word {
		idx := char - 'a'
		if node.children[idx] == nil {
			node.children[idx] = &Trie{}
		}
		node = node.children[idx]
	}
	node.isCompleteWord = true
}

func (this *Trie) SearchNode(word string) *Trie {
	node := this
	for _, char := range word {
		idx := char - 'a'
		if child := node.children[idx]; child != nil {
			node = child
		} else {
			return nil
		}
	}
	return node
}

func (this *Trie) Search(word string) bool {
	node := this.SearchNode(word)
	return node != nil && node.isCompleteWord
}

func (this *Trie) StartsWith(prefix string) bool {
	node := this.SearchNode(prefix)
	return node != nil
}
