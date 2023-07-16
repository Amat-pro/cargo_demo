package consistent_hash_code

import (
	"fmt"
	"hash/crc32"
	"sort"
)

type ConsistentHash struct {
	nodeNames []string // 实际节点+虚拟节点

	hashCircle []uint32 // 哈希环

	hashValueNodeNames map[uint32]string // key: nodeName's hash value   value: node name

}

func NewConsistentHash(names []string, virtualCount int) *ConsistentHash {

	if len(names) == 0 {
		panic("invalid names: cannot be empty")
	}

	if virtualCount <= 0 {
		panic("invalid virtual count: must gt zero")
	}

	dup, name := checkNamesDuplicated(names)
	if dup {
		panic(fmt.Sprintf("invalid names: %s duplicated", name))
	}

	nodeNames := make([]string, 0)
	hashCircle := make([]uint32, 0)
	hashValueNodeNames := make(map[uint32]string)

	for _, name := range names {

		for i := 0; i <= virtualCount; i++ {

			nodeNames = append(nodeNames, name)

			newName := fmt.Sprintf("%s#%d", name, i)
			hashValue := hash(newName)

			hashCircle = append(hashCircle, hashValue)
			hashValueNodeNames[hashValue] = name

		}

	}

	sort.Slice(hashCircle, func(i, j int) bool {
		return hashCircle[i] < hashCircle[j]
	})

	return &ConsistentHash{
		nodeNames:          nodeNames,
		hashCircle:         hashCircle,
		hashValueNodeNames: hashValueNodeNames,
	}
}

func (constantHash *ConsistentHash) Get(key string) string {

	hashValue := hash(key)

	// 比hashValue大的里面取最小的
	index := sort.Search(len(constantHash.hashCircle), func(i int) bool {
		return hashValue <= constantHash.hashCircle[i]
	})

	if index >= len(constantHash.hashCircle) {
		index = 0
	}

	hash := constantHash.hashCircle[index]
	return constantHash.hashValueNodeNames[hash]

}

func (constantHash *ConsistentHash) printHashCircle() {
	for _, hash := range constantHash.hashCircle {
		fmt.Printf("%s ", constantHash.hashValueNodeNames[hash])
	}
}

func hash(key string) uint32 {
	return crc32.ChecksumIEEE([]byte(key))
}

func checkNamesDuplicated(names []string) (bool, string) {
	if len(names) == 0 {
		return false, ""
	}

	tempMap := make(map[string]int)
	for _, name := range names {
		if _, ok := tempMap[name]; ok {
			return true, name
		}

		tempMap[name] = 0
	}

	return false, ""

}
