package hash

func isHappy(n int) bool {

	set := make(map[int]bool)
	num := n
	for num != 1 && !set[num] {
		num, set[num] = getSum(num), true
	}

	return num == 1
}

func getSum(n int) int {
	sum := 0
	for n != 0 {
		sum += (n % 10) * (n % 10)
		n = n / 10
	}

	return sum
}
