package bags

func combinationSum(nums []int, target int) int {

	// 完全背包

	// 定义dp
	dp := make([]int, target+1)
	// 初始化
	dp[0] = 1 // dp[i] = 0
	// 遍历
	// 求排列  先遍历背包
	for j := 0; j <= target; j++ { // 背包
		for i := 0; i < len(nums); i++ { // 物品
			if j >= nums[i] {
				dp[j] += dp[j-nums[i]]
			}
		}
	}
	return dp[target]
}
