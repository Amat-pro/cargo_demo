package num_is_lands

func numIslands(grid [][]byte) int {

	m := len(grid)
	if m == 0 {
		return 0
	}
	n := len(grid[0])

	// dfs: 遍历，遍历过的元素设置为2
	var dfs func(i, j int)
	dfs = func(i, j int) {

		// 确定终止条件
		// 越界
		if i < 0 || i >= m || j < 0 || j >= n {
			return
		}
		// 已经遍历过
		if grid[i][j] == 2 {
			return
		}
		// 是海域
		if grid[i][j] == 0 {
			return
		}

		// 单层递归的逻辑
		// 1.
		grid[i][j] = 2
		// 2.
		// 上
		dfs(i-1, j)
		// 下
		dfs(i+1, j)
		// 左
		dfs(i, j-1)
		// 右
		dfs(i, j+1)

	}

	res := 0
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if grid[i][j] == 1 {
				dfs(i, j)
				res++
			}
		}
	}

	return res
}
