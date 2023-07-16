package hash

import (
	"fmt"
	"testing"
)

func Test_isAnagram(t *testing.T) {
	s1 := "anagram"
	s2 := "nagaram"

	fmt.Println(isAnagram(s1, s2))

	fmt.Println(rune('a')) // 97
	fmt.Println()
	for _, r := range s1 {
		// fmt.Println(r)
		fmt.Println(r - rune('a'))
	}
}
