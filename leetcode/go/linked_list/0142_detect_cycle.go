package linked_list

// refer: https://github.com/Amat-pro/leetcode-master/blob/master/problems/0142.环形链表II.md
func detectCycle(head *ListNode) *int {

	slow, fast := head, head
	for fast != nil && fast.Next != nil {
		fast = fast.Next.Next
		slow = slow.Next

		if slow == fast {
			pos := 0
			for head != slow {
				head = head.Next
				slow = slow.Next
				pos++
			}

			return &pos

		}
	}

	return nil

}
