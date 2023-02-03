package dp

import "fmt"

func minCostClimbStairs(cost []int) int {
	// 定义dp
	dp := make([]int, len(cost)+1)

	// 初始化
	dp[0] = 0
	dp[1] = 0 // 可以从0或1开始跳 ！！！

	// 遍历
	for i := 2; i < len(cost)+1; i++ {
		// 递推公式
		dp[i] = minCost(dp[i-1]+cost[i-1], dp[i-2]+cost[i-2])
	}

	fmt.Println("dp: ", dp)
	return dp[len(cost)]

}

func minCost(a, b int) int {
	if a < b {
		return a
	}

	return b
}
