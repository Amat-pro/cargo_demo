package bags

import "math"

func numSquares(n int) int {

	// 完全背包

	// 物品
	nums := make([]int, 0) // 设置成n个物品
	for i := 0; i < n; i++ {
		nums = append(nums, (i+1)*(i+1))
	}

	// 定义dp
	dp := make([]int, n+1)
	// 初始化
	dp[0] = 0
	for i := 1; i <= n; i++ {
		dp[i] = math.MaxInt
	}

	// 遍历物品
	for i := 0; i < len(nums); i++ { // 物品
		for j := nums[i]; j <= n; j++ { // 背包
			if dp[j-nums[i]] != math.MaxInt {
				dp[j] = min(dp[j], dp[j-nums[i]]+1)
			}
		}
	}

	// 这里这一步可以不做判断，因为物品中nums[0] == 1,就一定存在dp[n] != math.MaxInt
	// if dp[n] == math.MaxInt {

	// }

	return dp[n]

}
