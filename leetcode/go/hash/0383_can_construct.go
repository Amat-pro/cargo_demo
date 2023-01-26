package hash

func canConstruct(s1, s2 string) bool {
	record := make([]int, 26)

	for _, v := range s2 {
		record[v-'a']++
	}

	for _, v := range s1 {
		record[v-'a']--
		if record[v-'a'] < 0 {
			return false
		}
	}

	return true
}
