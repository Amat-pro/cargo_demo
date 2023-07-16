package linked_list

func getIntersectionNode(headA, headB *ListNode) *ListNode {

	if headA.len() < headB.len() {
		// temp := headA
		// headA = headB
		// headB = temp
		headA, headB = headB, headA
	}

	var curA, curB *ListNode
	curB = headB
	curA = headA
	for i := 0; i < headA.len()-headB.len(); i++ {
		curA = curA.Next
	}

	// 遍历两个链表遇到相同则跳出遍历
	// 这里判断的是指针相等！！！！！！
	for curA != curB {
		curA = curA.Next
		curB = curB.Next
	}
	return curA

}
