package string

import (
	"fmt"
	"testing"
)

func Test_reverseLeftWords(t *testing.T) {
	fmt.Println("==> ", reverseLeftWords("abcdefg", 2))
	fmt.Println("==> ", "cdefgab")
	fmt.Println()
	fmt.Println("==> ", reverseLeftWords("lrloseumgh", 6))
	fmt.Println("==> ", "umghlrlose")
}
