package backtrack

import "fmt"

func solveShudu(board [][]byte) {
	var backtracking func(board [][]byte) bool
	backtracking = func(board [][]byte) bool {

		// 确定结束条件（当两层for结束时停止）

		// 单层处理逻辑
		for i := 0; i < 9; i++ {
			for j := 0; j < 9; j++ {
				if board[i][j] != '.' {
					continue
				} else {
					for _, z := range []byte{'1', '2', '3', '4', '5', '6', '7', '8', '9'} {
						if isNumValid(z, board, i, j) {
							board[i][j] = z
							// 递归
							result := backtracking(board)
							if result {
								return true
							}
							// 回溯
							board[i][j] = '.'
						}
					}
					return false
				}
			}
		}

		return true
	}

	result := backtracking(board)
	if result {
		fmt.Println(board)
	} else {
		fmt.Println("solveShudo result is false")
	}
}

func isNumValid(num byte, board [][]byte, i, j int) bool {
	// 行
	for col := 0; col < 9; col++ {
		if board[i][col] == num {
			return false
		}
	}
	// 列
	for row := 0; row < 9; row++ {
		if board[row][j] == num {
			return false
		}
	}

	// 方格
	startI := (i / 3) * 3
	startJ := (j / 3) * 3
	for z := startI; z < startI+3; startI++ {
		for k := startJ; k < startJ+3; startJ++ {
			if board[z][k] == num {
				return false
			}
		}
	}

	return true
}
