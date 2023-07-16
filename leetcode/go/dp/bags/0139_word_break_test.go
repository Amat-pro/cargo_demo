package bags

import (
	"fmt"
	"testing"
)

func Test_wordBreak(t *testing.T) {
	// true
	fmt.Println("==> ", wordBreak("leetcode", []string{"leet", "code"}))
	// true
	fmt.Println("==> ", wordBreak("applepenapple", []string{"apple", "pen"}))
	// false
	fmt.Println("==> ", wordBreak("catsandog", []string{"cats", "dog", "sand", "and", "cat"}))

}
