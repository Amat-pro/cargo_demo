package string

import (
	"fmt"
	"testing"
)

func Test_reverseWords(t *testing.T) {
	fmt.Println("==> ", reverseWords("the sky is blue"))
	fmt.Println("==> ", "blue is sky the")
	fmt.Println()
	fmt.Println("==> ", reverseWords("  hello world!  "))
	fmt.Println("==> ", "world! hello")
	fmt.Println()
	fmt.Println("==> ", reverseWords("a good   example"))
	fmt.Println("==> ", "example good a")
}
