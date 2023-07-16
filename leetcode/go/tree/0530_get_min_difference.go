package tree

import "math"

// 给你一棵所有节点为非负值的二叉搜索树，请你计算树中任意两节点的差的绝对值的最小值。

// 方法一
// 二叉搜索树通过中序遍历转化成单调递增数组，然后两两元素求差的最小绝对值

// 方法二 （双指针）
// 中序遍历
func getMinimumDifference(root *TreeNode) int {
	var preNode *TreeNode
	preNode = nil

	min := math.MaxInt64

	var travel func(node *TreeNode)
	travel = func(node *TreeNode) {
		if node == nil {
			return
		}

		// 左
		travel(node.Left)

		// 中
		if preNode != nil && getAbsoluteDiff(node.Val, preNode.Val) < min {
			min = getAbsoluteDiff(node.Val, preNode.Val)
		}
		preNode = node

		// 右
		travel(node.Right)

	}

	return min

}

func getAbsoluteDiff(v1, v2 int) int {
	result := v1 - v2
	if result >= 0 {
		return result
	}

	return -result
}
