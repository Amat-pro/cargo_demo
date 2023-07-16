package linked_list

import (
	"testing"
)

func Test_getIntersectionNode(t *testing.T) {
	head1 := newListNode(4)
	n2 := newListNode(1)
	n3 := newListNode(8)
	n4 := newListNode(4)
	n5 := newListNode(5)

	head1.Next = n2
	n2.Next = n3
	n3.Next = n4
	n4.Next = n5

	head2 := newListNode(5)
	n22 := newListNode(0)

	head2.Next = n22
	// 指针相等 ！！！
	n22.Next = n2

	head1.print()
	head2.print()

	getIntersectionNode(head1, head2).print()
}
