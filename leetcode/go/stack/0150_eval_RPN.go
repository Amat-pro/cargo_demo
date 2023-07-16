package stack

import (
	"strconv"
)

func evalRPN(tokens []string) int {

	myStack := []int{}

	var num int
	var num1, num2, length int
	for _, token := range tokens {
		length = len(myStack)
		switch token {
		case "+":
			num1 = myStack[length-2]
			num2 = myStack[length-1]
			myStack = myStack[:length-2]
			myStack = append(myStack, num1+num2)
		case "-":
			num1 = myStack[length-2]
			num2 = myStack[length-1]
			myStack = myStack[:length-2]
			myStack = append(myStack, num1-num2)
		case "*":
			num1 = myStack[length-2]
			num2 = myStack[length-1]
			myStack = myStack[:length-2]
			myStack = append(myStack, num1*num2)
		case "/":
			num1 = myStack[length-2]
			num2 = myStack[length-1]
			myStack = myStack[:length-2]
			myStack = append(myStack, num1/num2)
		default:
			// 数字
			num, _ = strconv.Atoi(token)
			myStack = append(myStack, num)
		}
	}

	return myStack[0]

}
