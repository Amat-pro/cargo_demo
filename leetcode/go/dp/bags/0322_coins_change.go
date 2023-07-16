package bags

import "math"

func coinsChange_i(coins []int, amount int) int {

	// 完全背包

	// 定义dp
	dp := make([]int, amount+1)
	// 初始化
	dp[0] = 0
	for i := 1; i <= amount; i++ {
		dp[i] = math.MaxInt
	}

	// 遍历
	for i := 0; i < len(coins); i++ { // 物品
		for j := coins[i]; j <= amount; j++ { // 背包
			// 递推公式
			// 需要判断下dp[j-coins[i]]  // !!!
			if dp[j-coins[i]] == math.MaxInt {
				continue
			} else {
				dp[j] = min(dp[j], dp[j-coins[i]]+1)
			}
		}
	}

	if dp[amount] == math.MaxInt {
		return -1
	}

	return dp[amount]
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
