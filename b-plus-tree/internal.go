package b_plus_tree

import "fmt"

const minOrder = 3

// indexNode index of data
// if I am an index node, then my structure would be seemed like this:
// (childs[0], indices[0], ... childs[i], indices[i], ..., childs[n-1], indices[n-1], childs[n])
// For any i, such that childs[i] <= indices[i]
type indexNode struct {
	parent  *indexNode   // parent of current node
	indices []int        // indices index of tree
	childs  []*indexNode // childs empty array if I am a data node for storing data
	vals    [][]string   // vals store actual data in the tree
	next    *indexNode
	isLeaf  bool
}

func (node *indexNode) insert(key int, value string, order int) {
	leaf := node.find(key)
	leaf.insertLeaf(key, value)
	leaf.split(order)
	return
}

func (node *indexNode) insertLeaf(key int, value string) {
	for i, index := range node.indices {
		if key == index {
			// append data
			node.vals[i] = append(node.vals[i], value)
			return
		} else if key < index {
			// insert data
			node.indices = insertNth(node.indices, key, i)
			node.vals = insertNth(node.vals, []string{value}, i)
			return
		}
	}

	// if we reach this position, it indicates that we have reach the end of the array, and the target key
	// is still less than the current index, then we should search the last child.
	// insert data
	node.indices = append(node.indices, key)
	node.vals = append(node.vals, []string{value})
}

func (node *indexNode) split(order int) {
	if node.shouldSplitLeaf(order) {

		var separatorIndex = getSeparatorIndex(order)

		right := newIndexNode(true)
		right.parent = node.parent
		right.indices = copyToEnd(node.indices, separatorIndex)
		right.vals = node.vals[separatorIndex:]
		right.next = node.next

		left := newIndexNode(true)
		left.parent = node.parent
		left.indices = node.indices[:separatorIndex]
		left.vals = node.vals[:separatorIndex]
		left.next = right

		var newIndex = right.indices[0]
		node.splitBranch(order, left, newIndex, right)
	}
}

// splitBranch separate the non-leaf node
func (node *indexNode) splitBranch(order int, left *indexNode, newIndex int, right *indexNode) {
	if node.parent == nil {
		// I am the root node
		left.parent = node
		right.parent = node
		node.isLeaf = false
		node.indices = []int{newIndex}
		node.childs = []*indexNode{left, right}
		return
	}

	// **now I suggest I am not the root node**

	// Considering the current data structure, each node represents a branch and
	// its position must correspond to one of the following three situations:
	// 1. node is in the far left position : `node` < `indices[0]`
	// 2. node is the middle position : `indices[i]` <= `node` < `indices[i+1]`
	// 3. node is the far right position : `indices[N]` <= `node`;
	// Assuming the index is `i`, then we have to do like as follows:
	// 1. find the `node` and its index which also is the index of indices;
	// 2. insert `left` and `right` into vals;
	// 3. insert the new `index` into the found position in `indices`.
	// after doing so, the structure will be transformed into one of the following structures:
	// 1. `left` < `newIndex` <= `right` < `indices[1]`
	// 2. `indices[i-1]` <= `left` < `newIndex` <= `right` < `indices[i+1]`
	// 3. `indices[N]` <= `left` < `newIndex` <= `right`
	parent := node.parent
	for index, child := range parent.childs {
		if child != node {
			continue
		}

		// set node to left
		parent.childs[index] = left
		// update next pointer of previous child
		parent.setNext(index, left)
		parent.indices = insertNth(parent.indices, newIndex, index)
		parent.childs = insertNth(parent.childs, right, index+1)

		if parent.shouldSplitBranch(order) {
			// now it's time to separate parent
			var separatorIndex = getSeparatorIndex(order)

			right = newIndexNode(false)
			// we are separating parent into two different nodes, which means the new node's parent is the old node's parent
			right.parent = parent.parent
			right.indices = parent.indices[separatorIndex:]
			right.childs = parent.childs[separatorIndex:]
			for _, child := range right.childs {
				child.parent = right
			}

			left = newIndexNode(false)
			left.parent = parent.parent
			left.indices = parent.indices[:separatorIndex]
			left.childs = parent.childs[:separatorIndex]
			for _, child := range left.childs {
				child.parent = left
			}

			var newBranchIndex = parent.indices[separatorIndex-1]
			parent.splitBranch(order, left, newBranchIndex, right)
		}

		break
	}
}

func (node *indexNode) find(key int) *indexNode {
	if node.isLeaf {
		return node
	}

	var i = 0
	for i = 0; i < len(node.indices); i++ {
		if key < node.indices[i] {
			return node.childs[i].find(key)
		}
	}

	// if we reach this position, it indicates that we have reach the end of the array, and the target key
	// is still less than the current node, then we should search the last child.
	//
	// if i == len(node.indices) {
	//
	// }
	return node.childs[i].find(key)
}

func (node *indexNode) search(key int) *Data {
	leaf := node.find(key)
	for i, index := range leaf.indices {
		if index == key {
			return toFinalData(i, leaf)
		}
	}
	return nil
}

func (node *indexNode) shouldSplitLeaf(order int) bool {
	return len(node.indices) == order
}

func (node *indexNode) shouldSplitBranch(order int) bool {
	return len(node.childs) == order
}

func (node *indexNode) setNext(index int, next *indexNode) {
	if index > 0 {
		// find previous sibling
		prevSibling := node.childs[index-1]
		for !prevSibling.isLeaf {
			prevSibling = prevSibling.childs[len(prevSibling.childs)-1]
		}

		// find next
		nextSibling := next
		for !nextSibling.isLeaf {
			nextSibling = nextSibling.childs[0]
		}
		prevSibling.next = nextSibling

		return
	}

	parent := node.parent
	if parent == nil {
		// I am the first node.
		return
	}

	for siblingIndex, sibling := range parent.childs {
		if sibling == node {
			// find the position of my parent
			parent.setNext(siblingIndex, next)
		}
	}
}

func (node *indexNode) print() {
	node.doPrint(0)
}

func (node *indexNode) doPrint(depth int) {
	if node.isLeaf {
		for j := 0; j < depth; j++ {
			fmt.Printf("    ")
		}
		fmt.Printf("[")
		for _, index := range node.indices {
			fmt.Printf("`%d`,", index)
		}
		fmt.Printf("]\n")
	} else {
		for i, child := range node.childs {
			for j := 0; j < depth; j++ {
				fmt.Printf("    ")
			}
			child.doPrint(depth + 1)

			if i == len(node.indices) {
				continue
			}
			fmt.Printf("%d\n", node.indices[i])
		}
	}
}
