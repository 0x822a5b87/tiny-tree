package b_plus_tree

import (
	"math/rand"
	"strconv"
	"testing"
	"time"
)

type input struct {
	key int
	val string
}

func TestNewBTree(t *testing.T) {
	order := 10
	root := NewBTree(order)
	if root.order != checkOrder(order) {
		t.Error("the order should be", order)
	}
}

func Test_RandomTest(t *testing.T) {
	for i := 0; i < 10000; i++ {
		minV, maxV := 3, 10
		order := rand.Intn(maxV-minV+1) + minV
		minV, maxV = 10, 500
		count := rand.Intn(maxV-minV+1) + minV
		testBTreeSearchShuffle(t, order, count)
	}
}

func Test_UnShuffleRandomTest(t *testing.T) {
	for i := 0; i < 100; i++ {
		minV, maxV := 3, 5
		order := rand.Intn(maxV-minV+1) + minV
		minV, maxV = 100, 1000
		count := rand.Intn(maxV-minV+1) + minV
		testBTreeSearchUnShuffle(t, order, count)
	}
}

func testBTreeSearchUnShuffle(t *testing.T, order, count int) {
	t.Helper()
	inputs := genData(count, false)

	root := NewBTree(order)
	for _, v := range inputs {
		root.Insert(v.key, v.val)
	}
	assertBalanced(t, root.order, root.root)

	for _, v := range inputs {
		data := &Data{
			Key: v.key,
			Val: []string{v.val},
		}
		compareData(t, root.Search(v.key), data)
	}

	assertLeafLinkedList(t, root, count, inputs)
}

func testBTreeSearchShuffle(t *testing.T, order, count int) {
	inputs := genData(count, true)

	root := NewBTree(order)

	none := root.Search(0)
	if none != nil {
		t.Errorf("expected nil, but got [%v]", none)
	}

	for _, v := range inputs {
		root.Insert(v.key, v.val)
	}

	assertBalanced(t, root.order, root.root)

	for _, v := range inputs {
		data := &Data{
			Key: v.key,
			Val: []string{v.val},
		}
		compareData(t, root.Search(v.key), data)
	}

	assertLeafLinkedList(t, root, count, inputs)
}

func assertLeafLinkedList(t *testing.T, root *BTree, count int, inputs []input) {
	start := root.root.find(0)
	for i := 0; i < count; {
		if start == nil {
			t.Errorf("the linked list is error, failed to search value = %v", i)
			return
		}
		for _, v := range start.indices {
			if v != i {
				t.Errorf("expected %d, actual %d", i, v)
				return
			}
			i++
		}
		start = start.next
	}
}

func assertBalanced(t *testing.T, order int, tree *indexNode) {
	t.Helper()

	if len(tree.indices) >= order {
		t.Errorf("the tree should have number of indices less than %d, got %d", order, len(tree.indices))
	}

	if len(tree.childs) >= order {
		t.Errorf("the tree should have number of childs less than %d, got %d", order, len(tree.childs))
	}

	if tree.isLeaf {
		indicesCnt, valsCnt := len(tree.indices), len(tree.vals)
		if indicesCnt != valsCnt {
			t.Errorf("expected indices count to be %d, got %d", valsCnt, indicesCnt)
		}
	}

	for _, child := range tree.childs {
		assertBalanced(t, order, child)
	}

	//if tree.isLeaf {
	//	return 1
	//}
	//
	//var height = -1
	//for _, child := range tree.childs {
	//	h := assertBalanced(t, order, child)
	//	if height < 0 {
	//		height = h
	//		continue
	//	}
	//	if height != h {
	//		t.Errorf("expected height to be %d, got %d", height, h)
	//	}
	//}
}

func compareDataList(t *testing.T, actual, expected []*Data) {
	t.Helper()
	if len(actual) != len(expected) {
		t.Errorf("expected [%d], but got [%d]", len(expected), len(actual))
		return
	}
	for i := 0; i < len(expected); i++ {
		compareData(t, expected[i], actual[i])
	}
}

func compareData(t *testing.T, actual, expected *Data) {
	t.Helper()
	if actual == nil {
		t.Errorf("expected [%v], actual nil", expected)
	}

	if expected.Key != actual.Key {
		t.Errorf("expected [%v], actual [%v]", expected.Key, actual.Key)
	}
}

func genData(count int, shuffle bool) []input {
	inputs := make([]input, 0)
	for i := 0; i < count; i++ {
		inputs = append(inputs, input{
			key: i,
			val: strconv.Itoa(i),
		})
	}
	if shuffle {
		source := rand.NewSource(time.Now().UnixNano())
		r := rand.New(source)
		r.Shuffle(len(inputs), func(i, j int) {
			inputs[i], inputs[j] = inputs[j], inputs[i]
		})
	}
	return inputs
}
