package trie

import "testing"

func TestTrie(t *testing.T) {
	// var assert func(b bool)
	// assert = func(b bool) {
	// 	if !b {
	// 		t.Fail()
	// 	}
	// }

	trie := Constructor()
	trie.Insert("apple")
	b1 := trie.Search("apple")   // return True
	b2 := !trie.Search("app")    // return False
	b3 := trie.StartsWith("app") // return True
	trie.Insert("app")
	b4 := trie.Search("app") // return True

	print(b1, b2, b3, b4)
}
