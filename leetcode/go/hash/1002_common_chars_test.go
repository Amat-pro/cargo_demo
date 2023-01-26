package hash

import (
	"fmt"
	"testing"
)

func Test_commonChars(t *testing.T) {
	words1 := []string{"bella", "label", "roller"}
	fmt.Println("==>> ", (commonChars(words1)))

	words2 := []string{"cool", "lock", "cook"}
	fmt.Println("==>> ", (commonChars(words2)))

	words3 := []string{}
	fmt.Println("==>> ", (commonChars(words3)))

	words4 := []string{"cool"}
	fmt.Println("==>> ", (commonChars(words4)))
}
