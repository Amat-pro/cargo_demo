package hash

func commonChars(words []string) []string {

	l := len(words)

	if l == 0 {
		return []string{}
	}

	record := [][26]int{}

	for i := 0; i < l; i++ {
		row := [26]int{}
		for j := 0; j < len(words[i]); j++ {
			// rune('a') = 97
			row[words[i][j]-97]++
		}
		record = append(record, row)
	}

	result := []string{}
	// 查找每一列的最小值
	for i := 0; i < 26; i++ {
		minVal := record[0][i]
		for j := 0; j < l; j++ {
			minVal = min(minVal, record[j][i])
		}
		if minVal == 0 {
			continue
		}
		char := string(rune(i + 97))
		for z := 0; z < minVal; z++ {
			result = append(result, char)
		}
	}

	return result
}

func min(a, b int) int {
	if a > b {
		return b
	}
	return a
}
