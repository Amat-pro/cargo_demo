package linked_list

import (
	"testing"
)

func Test_reverseList(t *testing.T) {
	heade1 := newListNode(6)
	n2 := newListNode(2)
	n3 := newListNode(6)
	n4 := newListNode(3)
	n5 := newListNode(4)
	n6 := newListNode(5)
	n7 := newListNode(6)
	heade1.Next = n2
	n2.Next = n3
	n3.Next = n4
	n4.Next = n5
	n5.Next = n6
	n6.Next = n7
	// 6 5 4 3 2 6
	reverseList(heade1).print()

	// ""
	var heade2 *ListNode
	reverseList(heade2).print()

	// "3"
	heade3 := newListNode(3)
	reverseList(heade3).print()
}
