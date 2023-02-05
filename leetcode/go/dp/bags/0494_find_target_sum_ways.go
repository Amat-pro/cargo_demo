package bags

func findTargetSumWays(nums []int, target int) int {
	// 01背包

	sum := 0
	for _, v := range nums {
		sum += v
	}

	if (target+sum)%2 == 1 {
		return 0
	}

	left := (target + sum) / 2

	// 1.定义dp
	dp := make([]int, left+1)
	// 2.初始化
	dp[0] = 1
	// 遍历
	for i := 0; i < len(nums); i++ {
		for j := left; j >= nums[i]; j-- {
			// 递推公式
			dp[j] += dp[j-nums[i]]
		}
	}

	return dp[left]

}
