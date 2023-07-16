package linked_list

func removeNthFromEnd(l *ListNode, nth int) *ListNode {

	if nth <= 0 {
		return l
	}

	zeroHead := newListNode(0)
	zeroHead.Next = l

	pre := zeroHead
	left := l
	right := l
	leftIdx, rightIdx := 1, 1

	for right != nil {
		if rightIdx-leftIdx == nth {
			right = right.Next
			left = left.Next
			pre = pre.Next

			rightIdx++
			leftIdx++
		} else {
			right = right.Next
			rightIdx++
		}
	}

	if rightIdx-leftIdx == nth {
		// 移除left
		pre.Next = left.Next
		return zeroHead.Next
	} else {
		// right指向nil了，但是不符合nth条件
		return l
	}

}
