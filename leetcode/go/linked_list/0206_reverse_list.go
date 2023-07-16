package linked_list

func reverseList(l *ListNode) *ListNode {

	var pre *ListNode
	var next *ListNode

	current := l

	for current != nil {
		next = current.Next

		current.Next = pre
		pre = current
		current = next

	}

	return pre
}
