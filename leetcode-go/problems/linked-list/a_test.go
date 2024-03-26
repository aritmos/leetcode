package linkedlist

import "testing"

func TestLruCache(t *testing.T) {
	lruCache := Constructor(2)

	getAssert := func(get, expected int) {
		if v := lruCache.Get(get); v != expected {
			t.Fatalf("Expected `%v`, got `%v`", expected, v)
		}
	}

	lruCache.Put(1, 0)
	lruCache.Put(2, 2)
	getAssert(1, 1)
	lruCache.Put(3, 3)
	getAssert(2, -1)
	lruCache.Put(4, 4)
	getAssert(1, -1)
	getAssert(3, 3)
	getAssert(4, 4)
}
