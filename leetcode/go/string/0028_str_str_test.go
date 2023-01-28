package string

import (
	"fmt"
	"testing"
)

func Test_strStr(t *testing.T) {
	fmt.Println("==>> ", strStr("hello", "ll"))
	fmt.Println("==>> ", strStr("aaaaa", "bba"))
	fmt.Println("==>> ", strStr("aaaa", "a"))
}

func Test_getNext(t *testing.T) {
	next := getNext("aabaab")
	// 0 1 0 1 2 3
	fmt.Println("==> ", next)
}
