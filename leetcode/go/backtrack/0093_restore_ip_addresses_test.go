package backtrack

import (
	"fmt"
	"testing"
)

func Test_restoreIpAddresses(t *testing.T) {
	// ["255.255.11.135","255.255.111.35"]
	fmt.Println("==> ", restoreIpAddresses("25525511135"))

	// ["0.0.0.0"]
	fmt.Println("==> ", restoreIpAddresses("0000"))

	// ["1.1.1.1"]
	fmt.Println("==> ", restoreIpAddresses("1111"))

	// ["0.10.0.10","0.100.1.0"]
	fmt.Println("==> ", restoreIpAddresses("010010"))

	// ["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]
	fmt.Println("==> ", restoreIpAddresses("101023"))
}
