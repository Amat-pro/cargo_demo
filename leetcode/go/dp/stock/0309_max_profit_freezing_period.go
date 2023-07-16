package stock

func maxProfitWithFreezingPeriod(prices []int) int {
	// 定义dp
	dp := make([][4]int, len(prices))
	// 初始化
	dp[0][0] = -prices[0]
	dp[0][1] = 0
	dp[0][2] = 0
	dp[0][3] = 0

	// 遍历
	for i := 1; i < len(prices); i++ {
		// 递推公式
		dp[i][0] = max(dp[i-1][0], max(dp[i-1][3]-prices[i], dp[i-1][1]-prices[i]))
		dp[i][1] = max(dp[i-1][1], dp[i-1][3])
		dp[i][2] = dp[i-1][0] + prices[i]
		dp[i][3] = dp[i-1][2]
	}

	l := len(prices) - 1
	return max(dp[l][0], max(dp[l][1], max(dp[l][2], dp[l][3])))
}
