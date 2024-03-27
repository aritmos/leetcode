package trie

import "sync"

type WordDictionary struct {
	children       [26]*WordDictionary
	isCompleteWord bool
}

func Constructor() WordDictionary {
	return WordDictionary{}
}

func (this *WordDictionary) AddWord(word string) {
	node := this
	for _, char := range word {
		idx := char - 'a'
		if node.children[idx] == nil {
			node.children[idx] = &WordDictionary{}
		}
		node = node.children[idx]
	}
	node.isCompleteWord = true
}

func (this *WordDictionary) Search(word string) bool {
	return search(this, word, 0)
}

func search(node *WordDictionary, word string, i int) bool {
	l := len(word)
	for i < l {
		if char := word[i]; char == '.' {
			i++
			ch := make(chan bool)
			wg := sync.WaitGroup{}
			for _, child := range node.children {
				if child != nil {
					wg.Add(1)
					go func() {
						defer wg.Done()
						res := search(child, word, i)
						if res {
							ch <- true
						}
					}()
				}
			}

			go func() {
				wg.Wait()
				close(ch)
			}()

			for range ch {
				return true
			}
			return false
		} else {
			i++
			idx := char - 'a'
			if node.children[idx] == nil {
				return false
			}
			node = node.children[idx]
		}
	}
	return node.isCompleteWord
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * obj := Constructor();
 * obj.AddWord(word);
 * param_2 := obj.Search(word);
 */
