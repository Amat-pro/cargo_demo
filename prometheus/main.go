package main

import (
	"fmt"
	"prometheus/push_gateway"
)

func main() {
	fmt.Println("Hello, world!")

	push_gateway.MockPush()

}
