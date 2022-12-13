package consitenthashcode

import (
	"fmt"
	"sort"
	"testing"
)

func TestConsistentHash_Get(_ *testing.T) {
	constantHash := NewConsistentHash([]string{"1000", "2000", "3000"}, 20)
	fmt.Printf("key: %s  node: %s \n", "1", constantHash.Get("1"))

	fmt.Printf("key: %s  node: %s \n", "2", constantHash.Get("2"))
	fmt.Printf("key: %s  node: %s \n", "3", constantHash.Get("3"))

	fmt.Printf("key: %s  node: %s \n", "4", constantHash.Get("4"))
	fmt.Printf("key: %s  node: %s \n", "5", constantHash.Get("5"))
	fmt.Printf("key: %s  node: %s \n", "6", constantHash.Get("6"))
	fmt.Printf("key: %s  node: %s \n", "7", constantHash.Get("7"))
	fmt.Printf("key: %s  node: %s \n", "8", constantHash.Get("8"))
	fmt.Printf("key: %s  node: %s \n", "9", constantHash.Get("9"))
	fmt.Printf("key: %s  node: %s \n", "10", constantHash.Get("10"))

}

func TestConsistentHash_printHashCircle(t *testing.T) {
	constantHash := NewConsistentHash([]string{"1000", "2000", "3000", "4000", "5000", "6000", "7000"}, 10)
	constantHash.printHashCircle()
}

func TestSortSearch(t *testing.T) {
	a := []int{1, 3, 6, 10, 15, 21, 28, 36, 45, 55}
	x := 6

	i := sort.Search(len(a), func(i int) bool { return a[i] >= x })
	if i < len(a) && a[i] == x {
		fmt.Printf("found %d at index %d in %v\n", x, i, a)
	} else {
		fmt.Printf("%d not found in %v\n", x, a)
	}
}
