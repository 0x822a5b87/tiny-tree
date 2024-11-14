# tiny-tree

Record a simple snippet of code appearing in mind.

## b-plus-tree

B+ tree, unlike a B-tree, has two orders, ‘a’ and ‘b’, one for the internal nodes and the other for the external (or leaf) nodes. 

B+ Trees contain two types of nodes:

- ***\*Internal Nodes:\**** Internal Nodes are the nodes that are present in at least n/2 record pointers, but not in the root node,
- ***\*Leaf Nodes:\**** Leaf Nodes are the nodes that have n pointers.

### The Structure of the Internal Nodes of a B+ Tree of Order `a` is as Follows

```mermaid
flowchart TD

subgraph Internal Node
	P1["P[1]"]:::front
	k1["k[1]"]:::op
	k2["..."]:::op
	k3["k[i - 1]"]:::op
	P2["P[i]"]:::front
	k4["k[i]"]:::op
	k5["..."]:::op
	k6["k[c-1]"]:::op
	P3["P[c]"]:::front
end

P1 --> X1{"x < = k[1]"}:::header
P2 --> X2{"k[i - 1] < x < k[i]"}:::header
P3 --> X3{"k[c - 1] < x"}:::header

classDef front 1,fill:#FFCCCC,stroke:#333;
classDef back fill:#969,stroke:#333;
classDef op fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5
classDef header fill: #696,color: #fff,font-weight: bold,padding: 10px;
```

- `P` stands for a pointer points to another non-leaf node or leaf node;
- `K` stands for the key-value parit of data, which is ordered;
- `a` stands for the order of a B+Tree;
- `c` stands for the maximum size of internal node, in other words, it compel `c <= a`;
- Each internal node has at most a tree pointer;
- The root node has, at least two tree pointers, while the other internal nodes have at least `ceil(a / 2)` tree pointers each;
- If an internal node has `c` pointer, **c <= a**, then it has `c - 1` key-value pairs.

### The structure of The Leaf Nodes of a B+Tree of Order 'b' is as Follows

```mermaid
flowchart TB

subgraph LeafNode
	subgraph Node1[" "]
	direction TB
  	K1["K[c - 1]"]
  	D1["D[c-1]"]:::front
	end
	subgraph Node["..."]
	end
	subgraph Node2[" "]
	direction TB
  	K3["k[i]"]
  	D3["D[i]"]:::front
	end
	subgraph Node4[" "]
	direction TB
  	K4["K1"]
  	D4["D1"]:::front
	end

	Pointer:::op
end

Pointer --> NextLeafNode

NextLeafNode["Next Leaf Node"]:::op




classDef front 1,fill:#FFCCCC,stroke:#333;
classDef back fill:#969,stroke:#333;
classDef op fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5
classDef header fill: #696,color: #fff,font-weight: bold,padding: 10px;
```

- `c` <= `b` and each `D[i]` is a data pointer points to a acutal record in the disk whose key-value pair is K[i] or to a disk file block containing that record;
- k1 < k2 < ...
- Each leaf node has at least `ceil(b / 2)`
- All leaf nodes are the same level.

### insert

```mermaid
block-beta
columns 9

space:1
block:rootNode:6
    ptr1
    node4["3"]
    ptr2
    node8["7"] 
    ptr3
    node12["11"]
end
space:2

space:9

block:leftNode:3
    node1["1"]
    node2["2"]
    node3["3"]
end

block:middleNode:3
    node5["5"]
    node6["6"]
    node7["7"]
end

block:rightNode:3
    node9["9"]
    node10["10"]
    node11["11"]
end

ptr1 --> leftNode
ptr2 --> middleNode
ptr3 --> rightNode

class ptr1 header
class ptr2 header
class ptr3 header

classDef front 1,fill:#FFCCCC,stroke:#333;
classDef back fill:#969,stroke:#333;
classDef op fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5
classDef header fill: #696,color: #fff,font-weight: bold,padding: 10px;
```

> insert 3

```mermaid
block-beta
columns 9

space:1
block:rootNode:6
    ptr1
    node4["3"]
    ptr2
    node8["7"] 
    ptr3
    node12["12"]
end
space:2

space:9

block:leftNode:3
    node1["1"]
    node2["2"]
    node3["3"]
    node13["3"]
end

block:middleNode:3
    node5["5"]
    node6["6"]
    node7["7"]
end

block:rightNode:3
    node9["9"]
    node10["10"]
    node11["11"]
end

ptr1 --> leftNode
ptr2 --> middleNode
ptr3 --> rightNode

class ptr1 header
class ptr2 header
class ptr3 header

classDef front 1,fill:#FFCCCC,stroke:#333;
classDef back fill:#969,stroke:#333;
classDef op fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5
classDef header fill: #696,color: #fff,font-weight: bold,padding: 10px;
```

> size of first leaf node exceed maximum size of leaf, start divide.

 ```mermaid
 block-beta
 columns 10
 
 space:2
 block:rootNode:6
     ptr1
     node4["2"]
     ptr2
     node14["3"]
     ptr3
     node8["7"] 
     ptr4
     node12["12"]
 end
 space:2
 
 space:10
 
 block:leftNode:2
     node1["1"]
     node2["2"]
 end
 
 block:leftNode2:2
     node3["3"]
     node13["3"]
 end
 
 block:middleNode:3
     node5["5"]
     node6["6"]
     node7["7"]
 end
 
 block:rightNode:3
     node9["9"]
     node10["10"]
     node11["11"]
 end
 
 ptr1 --> leftNode
 ptr2 --> leftNode2
 ptr3 --> middleNode
 ptr4 --> rightNode
 
 class ptr1 header
 class ptr2 header
 class ptr3 header
 class ptr4 header
 
 classDef front 1,fill:#FFCCCC,stroke:#333;
 classDef back fill:#969,stroke:#333;
 classDef op fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5
 classDef header fill: #696,color: #fff,font-weight: bold,padding: 10px;
 ```

> size of root node exceed maximum size of non-leaf node, start divide.

```mermaid
block-beta
columns 10

space:4
block:newRootNode:2
    ptr1
    ptr2
end
space:4

space:10

block:rootNode1:5
    ptr3
    node4["2"]
    ptr4
    node15["3"]
end

block:rootNode2:5
    ptr5
    node8["7"] 
    ptr6
    node12["11"]
end

space:10

block:leftNode:2
    node1["1"]
    node2["2"]
end

block:leftNode2:2
    node3["3"]
    node13["3"]
end

block:middleNode:3
    node5["5"]
    node6["6"]
    node7["7"]
end

block:rightNode:3
    node9["9"]
    node10["10"]
    node11["11"]
end

ptr1 --> rootNode1
ptr2 --> rootNode2
ptr3 --> leftNode
ptr4 --> leftNode2
ptr5 --> middleNode
ptr6 --> rightNode

class ptr1 header
class ptr2 header
class ptr3 header
class ptr4 header
class ptr5 header
class ptr6 header

classDef front 1,fill:#FFCCCC,stroke:#333;
classDef back fill:#969,stroke:#333;
classDef op fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5
classDef header fill: #696,color: #fff,font-weight: bold,padding: 10px;
```

### reference

- [Introduction of B+ Tree](https://www.geeksforgeeks.org/introduction-of-b-tree/)
- [Insertion in a B+ tree](https://www.geeksforgeeks.org/insertion-in-a-b-tree/)
- [Deletion in B+Trees](https://www.geeksforgeeks.org/deletion-in-b-tree/)

