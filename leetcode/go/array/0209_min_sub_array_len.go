package array

import "math"

// 暴力破解
func min_sub_array_len(nums []int, target int) int {

	l := len(nums)

	sum := 0
	result := 0
	tempL := 0
	for i := 0; i < l; i++ {
		sum = 0
		for j := i; j < l; j++ {
			sum += nums[j]
			if sum > target {
				break
			}
			if sum == target {
				tempL = (j - i + 1)
				if result == 0 {
					result = tempL
				} else {
					if tempL < result {
						result = tempL
					}
				}
			}
			continue
		}
	}
	return result
}

// 滑动窗口
// see: https://camo.githubusercontent.com/dd84aee84237ebb78cf7ffde58803dc03350a4071d0981b8add65d9c59199ac4/68747470733a2f2f636f64652d7468696e6b696e672e63646e2e626365626f732e636f6d2f676966732f3230392e2545392539352542462545352542412541362545362539432538302545352542302538462545372539412538342545352541442539302545362539352542302545372542422538342e676966
func min_sub_array_len_v2(nums []int, target int) int {

	l := len(nums)

	result := math.MaxInt
	sum := 0

	top := 0    // ++ -> sum要加
	bottom := 0 // ++ -> sum要减

	tempL := 0
	// top代表着滑动窗口的结尾
	// bottom代表着滑动窗口的开头
	// (bottom是不会越界的，因为当bottom向右移动前，sum会减掉bottom当前的值[sum最小会被减为0]，从而sum >= target不成立)
	for top < l {
		sum += nums[top]
		for sum >= target {
			if sum == target {
				tempL = top - bottom + 1
				if tempL < result {
					result = tempL
				}
			}
			sum -= nums[bottom]
			bottom++
		}
		top++
	}

	if result == math.MaxInt {
		return 0
	}
	return result
}
