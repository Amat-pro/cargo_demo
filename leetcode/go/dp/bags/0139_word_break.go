package bags

func wordBreak(s string, wordDict []string) bool {

	wordMap := make(map[string]bool)
	for _, v := range wordDict {
		wordMap[v] = true
	}

	// 定义dp
	dp := make([]bool, len(s)+1)
	// 初始化
	dp[0] = true

	// 遍历  （求排列）
	for i := 1; i <= len(s); i++ { // 背包
		for j := 0; j < i; j++ { // 物品
			if dp[j] && wordMap[s[j:i]] {
				dp[i] = true
			}
		}
	}

	return dp[len(s)]

}
