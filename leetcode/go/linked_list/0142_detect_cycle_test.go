package linked_list

import (
	"fmt"
	"testing"
)

func Test_detectCycle(t *testing.T) {
	head1 := newListNode(3)
	n2 := newListNode(2)
	n3 := newListNode(0)
	n4 := newListNode(4)

	head1.Next = n2
	n2.Next = n3
	n3.Next = n4
	n4.Next = n2

	// 1
	fmt.Println(*detectCycle(head1))
}
