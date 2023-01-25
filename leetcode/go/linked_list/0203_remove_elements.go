package linked_list

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func (l *ListNode) print() {
	for l != nil {
		fmt.Print(l.Val)
		fmt.Print("  ")
		l = l.Next
	}
	fmt.Println()
}

func newListNode(val int) *ListNode {
	return &ListNode{
		Val:  val,
		Next: nil,
	}
}

func removeElements(head *ListNode, val int) *ListNode {

	// 虚拟头节点
	zeroHead := newListNode(0)
	zeroHead.Next = head

	pre := zeroHead
	current := head
	var next *ListNode

	for current != nil {
		if current.Val == val {
			next = current.Next
			pre.Next = next
		}

		pre = current
		current = current.Next
	}

	return zeroHead.Next
}
