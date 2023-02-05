package bags

func coinsChange(coins []int, amount int) int {

	// 完全背包

	// 定义dp
	dp := make([]int, amount+1)
	// 初始化
	dp[0] = 1 // dp[i] = 0 (i > 0 && i < amount+1)

	// 遍历  (这里是求组合数，所以先遍历物品再遍历背包)
	for i := 0; i < len(coins); i++ { //物品
		for j := coins[i]; j <= amount; j++ {
			// 递推公式
			dp[j] += dp[j-coins[i]]
		}
	}

	return dp[amount]

}
