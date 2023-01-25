package linked_list

import (
	"testing"
)

func Test_removeNthFromEnd(t *testing.T) {

	heade1 := newListNode(1)
	n2 := newListNode(2)
	n3 := newListNode(3)
	n4 := newListNode(4)

	heade1.Next = n2
	n2.Next = n3
	n3.Next = n4
	// 1 2 4
	removeNthFromEnd(heade1, 2).print()

	// ""
	var heade2 *ListNode
	removeNthFromEnd(heade2, 1).print()

	// ""
	heade3 := newListNode(3)
	removeNthFromEnd(heade3, 1).print()

	heade5 := newListNode(6)
	n52 := newListNode(2)
	n53 := newListNode(6)
	n54 := newListNode(3)
	n55 := newListNode(4)
	n56 := newListNode(5)
	n57 := newListNode(6)
	heade5.Next = n52
	n52.Next = n53
	n53.Next = n54
	n54.Next = n55
	n55.Next = n56
	n56.Next = n57
	// 6 2 6 3 4 5 6
	removeNthFromEnd(heade5, 8).print()

	// 2 6 3 4 5 6
	removeNthFromEnd(heade5, 7).print()

}
