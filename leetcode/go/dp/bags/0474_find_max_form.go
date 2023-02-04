package bags

func findMaxForm(strs []string, m, n int) int {

	// 背包容量 m个0 n个1
	// dp[i][j]: i个0,j个1的背包最多装dp[i][j]个物品
	// dp[i][j] = max(dp[i][j], d[i-x][j-y]+1)

	// 定义dp
	dp := make([][]int, m+1)
	for i := 0; i <= m; i++ {
		dp[i] = (make([]int, n+1))
	}

	// 初始化
	dp[0][0] = 0 // 非0下标也初始化为0

	// 遍历
	for z := 0; z < len(strs); z++ { // 遍历物品
		// 计算当前strs[i]的x个0和y个1
		x := 0
		y := 0
		for _, v := range strs[z] {
			if v == '0' {
				x++
			} else {
				y++
			}
		}

		// 遍历背包 (i,j可以交换位置)
		for i := m; i >= x; i-- {
			for j := n; j >= y; j-- {
				dp[i][j] = max(dp[i][j], dp[i-x][j-y]+1)
			}
		}
	}

	return dp[m][n]

}
