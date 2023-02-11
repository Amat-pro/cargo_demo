package union_find

import (
	"fmt"
	"testing"
)

func TestUnionSet(t *testing.T) {

	set := [][]int{
		{10, 7},
		{2, 4},
		{5, 7},
		{1, 3},
		{8, 9},
		{1, 2},
		{5, 6},
		{2, 3},
		{3, 4},
		{7, 10},
		{8, 9},
	}

	unionSet := &UnionSet{}
	unionSet.Init(11)

	for _, v := range set {
		unionSet.Union(v[0], v[1])
	}

	// true
	fmt.Println("==> ", unionSet.Find(8) == unionSet.Find(9))
	fmt.Println("==> ", unionSet.Find(4) == unionSet.Find(1))
	fmt.Println("==> ", unionSet.Find(10) == unionSet.Find(6))

	fmt.Println()

	// false
	fmt.Println("==> ", unionSet.Find(10) == unionSet.Find(9))
	fmt.Println("==> ", unionSet.Find(10) == unionSet.Find(2))

}
