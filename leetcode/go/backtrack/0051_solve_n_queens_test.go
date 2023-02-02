package backtrack

import (
	"fmt"
	"testing"
)

func Test_solveNQueens(t *testing.T) {
	fmt.Println("==> ", solveNQueens(1))
	fmt.Println("==> ", solveNQueens(4))
}
