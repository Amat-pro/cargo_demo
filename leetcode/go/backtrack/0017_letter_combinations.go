package backtrack

func letterCombinations(digest string) []string {

	numMaps := map[rune]string{
		'2': "abc",
		'3': "def",
		'4': "ghi",
		'5': "jkl",
		'6': "mno",
		'7': "pqrs",
		'8': "tuv",
		'9': "wxyz",
	}

	collection := []string{}
	for _, v := range digest {
		collection = append(collection, numMaps[v])
	}

	result := []string{}
	if len(collection) == 0 {
		return result
	}

	path := []byte{}

	var backtracking func(idex int)
	backtracking = func(idex int) {
		if idex >= len(collection) {
			result = append(result, string(path[:]))
			return
		}

		for i := 0; i < len(collection[idex]); i++ {
			path = append(path, collection[idex][i])
			// 递归
			backtracking(idex + 1)
			// 回溯
			path = path[:len(path)-1]
		}
	}

	backtracking(0)
	return result

}
