package lru

import (
	"fmt"
	"testing"
)

func Test_lru(t *testing.T) {
	cache := Constructor(2)
	cache.Put(1, 1)                     // {1=1}
	cache.Put(2, 2)                     // {1=1, 2=2}
	fmt.Println("1 ==> ", cache.Get(1)) // 1
	cache.Put(3, 3)                     // {3=3, 1=1}
	fmt.Println("-1==> ", cache.Get(2)) // -1
	cache.Put(4, 4)                     // {4=4, 3=3}
	fmt.Println("-1==> ", cache.Get(1)) // -1
	fmt.Println("3 ==> ", cache.Get(3)) // 3
	fmt.Println("4 ==> ", cache.Get(4)) // 4
}
