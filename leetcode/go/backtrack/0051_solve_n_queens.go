package backtrack

func solveNQueens(n int) [][][]string {
	result := [][][]string{}

	if n <= 0 {
		return result
	}

	// 初始化棋盘
	chessboard := [][]string{}
	for i := 0; i < n; i++ {
		row := []string{}
		for j := 0; j < n; j++ {
			row = append(row, ".")
		}
		chessboard = append(chessboard, row)
	}

	var backtracking func(row int)
	backtracking = func(row int) {
		// 确定终止条件
		// 收集结果
		if row == n {
			result = appendChessboard(result, chessboard)
			return
		}

		// 单层处理逻辑
		for i := 0; i < n; i++ {
			if is_valid(chessboard, row, i, n) {
				chessboard[row][i] = "Q"
				// 递归
				backtracking(row + 1)
				// 回溯
				chessboard[row][i] = "."
			}
		}

	}

	backtracking(0)
	return result

}

func is_valid(chessboard [][]string, row int, col int, n int) bool {
	// 同一列
	for i := 0; i < row; i++ {
		if chessboard[i][col] == "Q" {
			return false
		}
	}

	// 左上对角线
	for i, j := row-1, col-1; i >= 0 && j >= 0; i, j = i-1, j-1 {
		if chessboard[i][j] == "Q" {
			return false
		}
	}

	// 右上对角线
	for i, j := row-1, col+1; i >= 0 && j < n; i, j = i-1, j+1 {
		if chessboard[i][j] == "Q" {
			return false
		}
	}

	return true
}

func appendChessboard(result [][][]string, chessboard [][]string) [][][]string {
	chessboard_new := make([][]string, 0, len(chessboard))
	for _, v := range chessboard {
		temp := make([]string, len(v))
		copy(temp, v)
		chessboard_new = append(chessboard_new, temp)
	}
	result = append(result, chessboard_new)
	return result
}
