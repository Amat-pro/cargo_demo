package num_is_lands

import (
	"fmt"
	"testing"
)

func Test_numIslands(t *testing.T) {
	grid1 := [][]byte{
		{1, 1, 1, 1, 0},
		{1, 1, 0, 1, 0},
		{1, 1, 0, 0, 0},
		{0, 0, 0, 0, 0},
	}
	// 1
	fmt.Println("==> ", numIslands(grid1))
	fmt.Println(grid1)

	grid2 := [][]byte{
		{1, 1, 0, 0, 0},
		{1, 1, 0, 0, 0},
		{0, 0, 1, 0, 0},
		{0, 0, 0, 1, 1},
	}
	// 3
	fmt.Println("==> ", numIslands(grid2))
	fmt.Println(grid1)

	grid3 := [][]byte{
		{1, 1, 1, 1, 0},
		{1, 1, 0, 1, 0},
		{1, 1, 0, 0, 0},
		{0, 0, 0, 0, 0},
	}
	// 3
	fmt.Println("==> ", numIslands(grid3))
	fmt.Println(grid1)

}
