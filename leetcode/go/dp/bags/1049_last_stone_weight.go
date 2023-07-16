package bags

func lastStoneWeight(stones []int) int {
	// 01背包
	// 物品         重量       价值
	// 石头0     stones[0]  stones[0]
	// 石头i     stones[0]  stones[0]

	// 找和最接近的两堆石头 -> sum / 2 这两堆石头相撞产生的最后一块石头重量最小

	// 1.
	// dp[j]: 表示容量为j的背包最大价值为dp[j]
	// 2.
	// dp[j] = max(dp[j], dp[j-stones[i]]+stones[i]])
	// 3.
	// dp[0] = 0  dp[i] = 0(i > 0 && i < sum/2)  初始化为0: 因为初始化为0对这里的max逻辑不产生影响
	// 4.
	// for i := 0; i < len(stone); i++
	//     for j := sum/2; j >= stones[i]; i--

	sum := 0
	for _, v := range stones {
		sum += v
	}

	target := sum / 2

	// 定义dp
	dp := make([]int, target+1)
	// 初始化
	// ... 默认为0
	// 遍历
	for i := 0; i < len(stones); i++ {
		for j := target; j >= stones[i]; j-- {
			// 递推公式
			dp[j] = max(dp[j], dp[j-stones[i]]+stones[i])
		}
	}

	value := dp[target]

	return (sum - dp[target]) - value
}
