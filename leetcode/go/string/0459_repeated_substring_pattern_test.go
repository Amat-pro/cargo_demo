package string

import (
	"fmt"
	"testing"
)

func Test_repeatedSubstringPattern(t *testing.T) {
	fmt.Println("==> ", repeatedSubstringPattern("abab"))
	fmt.Println("==> ", repeatedSubstringPattern("aba"))
	fmt.Println("==> ", repeatedSubstringPattern("abcabca"))
	fmt.Println("==> ", repeatedSubstringPattern("abcabc"))

	fmt.Println()

	fmt.Println("==> ", repeatedSubstringPatternKMP("abab"))
	fmt.Println("==> ", repeatedSubstringPatternKMP("aba"))
	fmt.Println("==> ", repeatedSubstringPatternKMP("abcabca"))
	fmt.Println("==> ", repeatedSubstringPatternKMP("abcabc"))

	fmt.Println("====> ", getNext_0459("abab"))
	fmt.Println("====> ", getNext("abab"))
}
