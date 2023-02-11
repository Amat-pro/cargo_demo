// LRU: Least Recently User
// 最近最少使用算法
package lru

type LRUCache struct {
	capacity int
	keyMap   map[int]*Node     // 哈希表
	list     *DoubleLinkedList // 双向链表
}

func (cache *LRUCache) Get(key int) int {
	node, exist := cache.keyMap[key]
	if !exist {
		return -1
	}

	cache.makeRecently(key)
	return node.value
}

func (cache *LRUCache) Put(key, value int) {
	_, exist := cache.keyMap[key]
	if exist {
		cache.remove(key)
		cache.add(key, value)
	} else {
		if cache.capacity == cache.list.size {
			cache.deleteLRU()
		}
		cache.add(key, value)
	}
}

func (cache *LRUCache) makeRecently(key int) {

	node := cache.keyMap[key]

	// 删除节点
	cache.list.remove(node)

	// 添加节点
	cache.list.addFirst(node)

}

func (cache *LRUCache) add(key, value int) {
	node := createNode(key, value)

	newNode := cache.list.addFirst(node)
	cache.keyMap[key] = newNode

}

func (cache *LRUCache) remove(key int) {
	node := cache.keyMap[key]

	cache.list.remove(node)
	delete(cache.keyMap, key)

}

func (cache *LRUCache) deleteLRU() {
	node := cache.list.removeLast()
	delete(cache.keyMap, node.key)
}

func Constructor(capacity int) LRUCache {
	return LRUCache{
		capacity: capacity,
		keyMap:   make(map[int]*Node),
		list:     initDoubleLinkedList(),
	}
}

type Node struct {
	key   int
	value int

	pre  *Node
	next *Node
}

func createNode(key, value int) *Node {
	return &Node{
		key:   key,
		value: value,
	}
}

type DoubleLinkedList struct {
	head *Node // 虚拟头节点
	tail *Node // 虚拟的尾节点

	size int
}

func initDoubleLinkedList() *DoubleLinkedList {
	list := &DoubleLinkedList{
		head: createNode(0, 0),
		tail: createNode(0, 0),
		size: 0,
	}

	list.head.next = list.tail
	list.tail.pre = list.head

	return list
}

func (list *DoubleLinkedList) remove(node *Node) *Node {
	pre := node.pre
	next := node.next

	pre.next = next
	next.pre = pre

	list.size--

	return node
}

func (list *DoubleLinkedList) addFirst(node *Node) *Node {

	list.head.next.pre = node
	node.next = list.head.next

	list.head.next = node
	node.pre = list.head

	list.size++

	return node

}

func (list *DoubleLinkedList) removeLast() *Node {

	node := list.tail.pre

	node.pre.next = list.tail
	list.tail.pre = node.pre

	list.size--

	return node
}
