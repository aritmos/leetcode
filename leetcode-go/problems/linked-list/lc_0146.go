package linkedlist

// LRU Cache

type LRUCache struct {
	capacity int
	length   int
	nodeMap  map[int]*node // map into the list to get O(1) reads
	preHead  *node         // linked list to encode ordering
	tail     *node
}

type node struct {
	key   int
	value int
	prev  *node
	next  *node
}

func Constructor(capacity int) LRUCache {
	return LRUCache{
		capacity: capacity,
		nodeMap:  make(map[int]*node),
		preHead:  &node{},
	}
}

// move a *node already in the cache to the front
func (this *LRUCache) moveToHead(node_ptr *node) {
	if this.preHead.next != node_ptr {
		if node_ptr == this.tail { // tail node
			this.tail = node_ptr.prev
			this.tail.next = nil
		} else { // some middle node_ptr
			node_ptr.prev.next = node_ptr.next
			node_ptr.next.prev = node_ptr.prev
		}
		node_ptr.prev = this.preHead
		node_ptr.next = this.preHead.next
		this.preHead.next.prev = node_ptr
		this.preHead.next = node_ptr
	}
}

// add a *node with set `key` and `value` fields to the front of the cache
func (this *LRUCache) addToHead(node_ptr *node) {
	node_ptr.prev = this.preHead

	if this.length != 0 {
		node_ptr.next = this.preHead.next
		this.preHead.next.prev = node_ptr
	} else {
		this.tail = node_ptr
	}
	this.preHead.next = node_ptr
	this.nodeMap[node_ptr.key] = node_ptr
}

func (this *LRUCache) Get(key int) int {
	node, ok := this.nodeMap[key]
	if !ok {
		return -1
	}
	this.moveToHead(node)
	return node.value
}

func (this *LRUCache) Put(key int, value int) {
	node_ptr, seen := this.nodeMap[key]
	if seen {
		node_ptr.value = value
		this.moveToHead(node_ptr)
		return
	}

	// add new head node
	head := &node{
		key:   key,
		value: value,
	}

	this.addToHead(head)

	if this.length < this.capacity {
		this.length++
	} else {
		// remove oldest node
		old_node := this.tail
		this.tail = this.tail.prev
		this.tail.next = nil
		delete(this.nodeMap, old_node.key)
	}
}
