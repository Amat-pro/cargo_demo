package bags

import "fmt"

func canPartition(nums []int) bool {
	// 01背包
	//物品       重量     价值
	//  0      nums[0]  nums[0]
	//  1      nums[1]  nums[1]
	//  2      nums[2]  nums[2]
	//  i      nums[i]  nums[i]

	// 1.
	// dp[j]: 表示背包容量为j的最大价值为dp[j]
	// 2.
	// dp[j] = max(d[j], dp[j-nums[i]]+nums[i])
	// 3.
	// dp[0] = 0, dp[i] = 0 (i > 0 && i <= sum{nums})
	// 4.
	// for 物品 ++
	//     for 背包 --

	sum := 0
	for _, v := range nums {
		sum += v
	}

	if sum%2 == 1 { // sum是奇数，则不可能分割为两个相等的子集
		return false
	}

	// 定义dp
	dp := make([]int, (sum/2)+1)
	// 初始化
	//  这里全部初始化为0即可

	// 遍历
	for i := 0; i < len(nums); i++ { // 物品
		fmt.Printf("i: %d ", i)
		for j := (sum / 2); j >= nums[i]; j-- { // 背包  （j小于物品i的重量时直接结束即可）
			// 递推
			dp[j] = max(dp[j], dp[j-nums[i]]+nums[i])
			fmt.Printf(" %d:%d ", j, dp[j])
		}
		fmt.Println()
	}

	return dp[sum/2] == sum/2
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
