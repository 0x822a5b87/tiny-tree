package b_plus_tree

import "fmt"

func newIndexNode(isLeaf bool) *indexNode {
	return &indexNode{
		parent:  nil,
		indices: make([]int, 0),
		childs:  make([]*indexNode, 0),
		vals:    make([][]string, 0),
		next:    nil,
		isLeaf:  isLeaf,
	}
}

func toFinalData(index int, leaf *indexNode) *Data {
	key := leaf.indices[index]
	vals := leaf.vals[index]

	return &Data{
		Key: key,
		Val: vals,
	}
}

func getSeparatorIndex(order int) int {
	// we use this method just in case that we might want to manipulate the separating operation one day.
	// for example : if there are five elements in a node waiting to be separate into two nodes,
	// we might want control which side should have more elements, in this case, this method works.
	return order / 2
}

func checkOrder(order int) int {
	if order < minOrder {
		panic(fmt.Errorf("order must greater than %d", minOrder))
	}

	// it intends to assure the separating operation happens only
	// at the moment when a node contains even numbers of elements.
	// this contributes to maintain a more balanced tree.
	if order%2 != 0 {
		order += 1
	}

	return order
}

func insertNth[T any](elements []T, val T, nth int) []T {
	data := make([]T, len(elements)+1)
	copy(data[:nth], elements[:nth])
	data[nth] = val
	copy(data[nth+1:], elements[nth:])
	return data
}

func copySlice[T any](elements []T, start, end int) []T {
	dst := make([]T, end-start)
	copy(dst, elements[start:end])
	return dst
}

func copyFromStart[T any](elements []T, end int) []T {
	return copySlice(elements, 0, end)
}

func copyToEnd[T any](elements []T, start int) []T {
	return copySlice(elements, start, len(elements))
}
