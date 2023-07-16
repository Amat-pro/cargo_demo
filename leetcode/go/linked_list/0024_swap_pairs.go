package linked_list

func swapPairs(l *ListNode) *ListNode {

	zeroHead := newListNode(0)
	zeroHead.Next = l

	current := l

	var tempNode *ListNode
	var next *ListNode
	var third *ListNode
	var pre *ListNode

	pre = zeroHead
	for current != nil && current.Next != nil {
		// 确定current  next  third
		next = current.Next
		third = next.Next

		// -- 开始交换
		tempNode = current
		current = next
		next = tempNode
		// -- 修改指向
		current.Next = next
		next.Next = third
		pre.Next = current
		// -- 向前推进
		current = third
		pre = next
	}

	return zeroHead.Next

}
