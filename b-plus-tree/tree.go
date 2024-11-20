package b_plus_tree

func NewBTree(order int) *BTree {
	return &BTree{
		order: checkOrder(order),
		root:  newIndexNode(true),
	}
}

type BTree struct {
	order int
	root  *indexNode
}

func (t *BTree) Search(key int) *Data {
	return t.root.search(key)
}

func (t *BTree) Insert(key int, value string) {
	t.root.insert(key, value, t.order)
}

func (t *BTree) Remove(key int) {}

type Data struct {
	Key int
	Val []string
}
