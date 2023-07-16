package stock

func maxProfitWithCommission(prices []int, free int) int {
	// 定义dp
	dp := make([][2]int, len(prices))
	// 初始化   (其他初始化为0，因为如果收益是负数了题目要求返回0)
	dp[0][0] = -prices[0]
	dp[0][1] = 0

	// 遍历
	for i := 1; i < len(prices); i++ {
		// 递推公式
		dp[i][0] = max(dp[i-1][0], dp[i-1][1]-prices[i])
		dp[i][1] = max(dp[i-1][1], dp[i-1][0]+prices[i]-free)
	}

	return max(dp[len(prices)-1][0], dp[len(prices)-1][1])
}
