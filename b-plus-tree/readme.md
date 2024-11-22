# readme

> [Here is another explanation of B+Tree in java version](./java-code-with-explanation.md)

## Performance Optimization Plan

- ❌ Using a red-black tree to replace the array inside the B+Tree in order to decrease the performance of insert and delete operations.
- ❌ Using `[]byte` to replace the `slice` inside in order to save the memory usage.
- ✅ fix severe BUG : **leaf nodes could become invalid under certain special conditions.**√

## Deletion on B+ Tree

### The key to be deleted was not found

> Return and do nothing.

### Leaf nodes are still suitable after deletion

> Delete the corresponding key and return.

```mermaid
block-beta
columns 9

space:2

block:rootNode:5
    ptr1["p"]
    index1["4"]
    ptr2["p"]
    index2["7"]
    ptr3["p"]
end
space:2

space:9

block:left:3
	node1["1"]
	node2["2"]
	node3["3"]
end

block:mid:3
	node4["4"]
	node5["5"]
	node6["6"]
end

block:right:3
	node7["7"]
	node8["8"]
	node9["9"]
end

class ptr1 header
class ptr2 header
class ptr3 header

class index1 front
class index2 front

ptr1 --> left
ptr2 --> mid
ptr3--> right


classDef front 1,fill:#FFCCCC,stroke:#333;
classDef back fill:#969,stroke:#333;
classDef op fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5
classDef header fill: #696,color: #fff,font-weight: bold,padding: 10px;
```

> Delete `4`

```mermaid
block-beta
columns 9

space:2

block:rootNode:5
    ptr1["p"]
    index1["4"]
    ptr2["p"]
    index2["7"]
    ptr3["p"]
end
space:2

space:9

block:left:3
	node1["1"]
	node2["2"]
	node3["3"]
end

block:mid:3
	node5["5"]
	node6["6"]
end

block:right:3
	node7["7"]
	node8["8"]
	node9["9"]
end

class ptr1 header
class ptr2 header
class ptr3 header

class index1 front
class index2 front

ptr1 --> left
ptr2 --> mid
ptr3--> right


classDef front 1,fill:#FFCCCC,stroke:#333;
classDef back fill:#969,stroke:#333;
classDef op fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5
classDef header fill: #696,color: #fff,font-weight: bold,padding: 10px;
```

### Leaf nodes are less than minimum size after deletion

> One of the following steps should be taken if the node underflow :
>
> - Get a key by borrowing it from a `sibling` node if it contains more keys than the required minimum.
> - If the minimal number of keys is met by all of the sibling nodes, merge the underflow node with one of its siblings and modify the parent node as necessary.
>
> The principle is to maintain the balance of the tree after deletion.

#### Borrowing from sibling

```mermaid
block-beta
columns 9

space:2

block:rootNode:5
    ptr1["p"]
    index1["4"]
    ptr2["p"]
    index2["7"]
    ptr3["p"]
end
space:2

space:9

block:left:3
	node1["1"]
	node2["2"]
	node3["3"]
end

block:mid:3
	node4["4"]
	node5["5"]
end

block:right:3
	node7["7"]
	node8["8"]
	node9["9"]
end

class ptr1 header
class ptr2 header
class ptr3 header

class index1 front
class index2 front

ptr1 --> left
ptr2 --> mid
ptr3--> right


classDef front 1,fill:#FFCCCC,stroke:#333;
classDef back fill:#969,stroke:#333;
classDef op fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5
classDef header fill: #696,color: #fff,font-weight: bold,padding: 10px;
```

> Delete `4`：
>
> 1. Borrowing a node from siblings, and the procedure of borrowing might reconstruct the whole node;
> 2. Pop the change to their parent in order to reconstruct the indices, fortunately, the grandparent will remains the same as the reconstruction of indices doesn't decrease the minimum value or increase the maximum value, thus maintaining the balance of the tree.

```mermaid
block-beta
columns 9

space:2

block:rootNode:5
    ptr1["p"]
    index1["3"]
    ptr2["p"]
    index2["7"]
    ptr3["p"]
end
space:2

space:9

block:left:3
	node1["1"]
	node2["2"]
end

block:mid:3
	node3["3"]
	node5["5"]
end

block:right:3
	node7["7"]
	node8["8"]
	node9["9"]
end

class ptr1 header
class ptr2 header
class ptr3 header

class index1 front
class index2 front

ptr1 --> left
ptr2 --> mid
ptr3--> right


classDef front 1,fill:#FFCCCC,stroke:#333;
classDef back fill:#969,stroke:#333;
classDef op fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5
classDef header fill: #696,color: #fff,font-weight: bold,padding: 10px;
```

#### underflow met by all of the sibling

```mermaid
block-beta
columns 9

space:2

block:rootNode:5
    ptr1["p"]
    index1["4"]
    ptr2["p"]
    index2["7"]
    ptr3["p"]
end
space:2

space:9

block:left:3
	node1["1"]
	node3["3"]
end

block:mid:3
	node4["4"]
	node5["5"]
end

block:right:3
	node7["7"]
	node9["9"]
end

class ptr1 header
class ptr2 header
class ptr3 header

class index1 front
class index2 front

ptr1 --> left
ptr2 --> mid
ptr3--> right


classDef front 1,fill:#FFCCCC,stroke:#333;
classDef back fill:#969,stroke:#333;
classDef op fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5
classDef header fill: #696,color: #fff,font-weight: bold,padding: 10px;
```

> Delete `3`

```mermaid
block-beta
columns 6


block:rootNode:6
    ptr1["p"]
    index2["7"]
    ptr3["p"]
end

space:6

block:left:3
	node1["1"]
	node3["4"]
	node5["5"]
end

space:2

block:right:3
	node7["7"]
	node9["9"]
end

class ptr1 header
class ptr2 header
class ptr3 header

class index1 front
class index2 front

ptr1 --> left
ptr3--> right


classDef front 1,fill:#FFCCCC,stroke:#333;
classDef back fill:#969,stroke:#333;
classDef op fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5
classDef header fill: #696,color: #fff,font-weight: bold,padding: 10px;
```

