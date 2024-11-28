# Red Black Tree and AVL

## Red Black Tree

### insert data to a `2-` node

```mermaid
---
title: new value is smaller than old one
---
flowchart TB

B:::child --> C:::red
B:::black -.-> nil:::leaf

classDef red 1,fill:#FF4D4D,stroke:#333,background-color:#ff4d4d;
classDef black fill:#000,stroke:none,stroke-width:2px,color:#fff;
classDef leaf fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5;
```

```mermaid
---
title: new value is greater than old one
---
flowchart LR

subgraph before
	direction TB
	B:::child -.-> nil:::leaf
	B:::black --> C:::red
end

subgraph after
	direction TB
	C1["C"]:::black --> B1["B"]:::red
	C1 -.-> nil1["nil"]:::leaf
end

before -->|rotate| after

classDef red 1,fill:#FF4D4D,stroke:#333,background-color:#ff4d4d;
classDef black fill:#000,stroke:none,stroke-width:2px,color:#fff;
classDef leaf fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5;
```

> 1. the new node is always a `red` node;
> 2. if there is a right-side red link, we rotate it to get a left-side red link.

```mermaid
---
title: insert dat to a 2- node at the bottom of the tree
---
flowchart LR

subgraph first
	direction TB
	E1["E"]:::black
	A1["A"]:::black
	S1["S"]:::black
	R1["R"]:::red
	nil1["nil"]:::leaf

	E1 --> A1
	E1 --> S1
	S1 --> R1
	S1 --> nil1["nil"]
end

subgraph second
	direction TB
	E2["E"]:::black
	A2["A"]:::black
	S2["S"]:::black
	R2["R"]:::red
	C2["C"]:::red
	nil2_1["nil"]:::leaf

	E2 --> A2
	A2--> nil2_2["nil"]:::leaf
	A2 --> C2
	E2 --> S2
	S2 --> R2
	S2 --> nil2_1["nil"]
end

subgraph third
	direction TB
	E3["E"]:::black
	A3["A"]:::red
	S3["S"]:::black
	R3["R"]:::red
	C3["C"]:::black
	nil3_1["nil"]:::leaf

	E3 --> C3
	C3 --> A3
	C3--> nil3_3["nil"]:::leaf
	E3 --> S3
	S3 --> R3
	S3 --> nil3_1["nil"]
end

first -->|insert C| second -->|rotate| third

classDef red 1,fill:#FF4D4D,stroke:#333,background-color:#ff4d4d;
classDef black fill:#000,stroke:none,stroke-width:2px,color:#fff;
classDef leaf fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5;
```

### insert data to a `3-` node

> new node is the greatest one

```mermaid
flowchart LR

subgraph first
	direction TB

	a1["a"]
	b1["b"]
	nil1_1["nil"]

	b1:::black --> a1:::red
	b1 --> nil1_1:::leaf
end

subgraph second
	direction TB

	a2["a"]:::red
	b2["b"]:::black
	c2["c"]:::red

	b2 --> a2
	b2 --> c2
end

subgraph third
	direction TB

	a3["a"]:::black
	b3["b"]:::red
	c3["c"]:::black

	b3 --> a3
	b3 --> c3
end

first -->|insert C| second -->|rotate| third

classDef red 1,fill:#FF4D4D,stroke:#333,background-color:#ff4d4d;
classDef black fill:#000,stroke:none,stroke-width:2px,color:#fff;
classDef leaf fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5;
```

> new node is the smallest one

```mermaid
flowchart LR

subgraph first
	direction TB

	c1["c"]:::black
	b1["b"]:::red
	nil1_1["nil"]:::leaf

	c1 --> b1
	c1 --> nil1_1
end

subgraph second
	direction TB

	c2["c"]:::black
	b2["b"]:::red
    a2["a"]:::red
	nil2_1["nil"]:::leaf
	nil2_2["nil"]:::leaf

	c2 --> b2
	c2 --> nil2_1

    b2 --> a2
    b2 --> nil2_2
end

subgraph third
	direction TB

	c3["a"]:::red
	b3["b"]:::black
    a3["c"]:::red

    b3 --> a3
    b3 --> c3
end

subgraph fourth
	direction TB

	c4["a"]:::black
	b4["b"]:::red
    a4["c"]:::black

    b4 --> a4
    b4 --> c4
end

first --> |insert a| second -->|rotate| third -->|rotate| fourth

classDef red 1,fill:#FF4D4D,stroke:#333,background-color:#ff4d4d;
classDef black fill:#000,stroke:none,stroke-width:2px,color:#fff;
classDef leaf fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5;
```

> new node is the medium one:

```mermaid
flowchart LR

subgraph first
	direction TB

	a1["a"]:::red
	c1["c"]:::black
	nil1["nil"]:::leaf

	c1 --> a1
	c1 --> nil1
end

subgraph second
	direction TB

	a2["a"]:::red
	c2["c"]:::black
    b2["b"]:::red
	nil2["nil"]:::leaf
	nil2_1["nil"]:::leaf

	c2 --> a2
	c2 --> nil2

    a2 --> nil2_1
    a2 --> b2
end

subgraph third
	direction TB

	a3["a"]:::red
	c3["c"]:::black
    b3["b"]:::red
	nil3["nil"]:::leaf
	nil3_1["nil"]:::leaf

	c3 --> b3
	c3 --> nil3

    b3 --> a3
    b3 --> nil3_1
end

subgraph fourth
	direction TB

	c4["a"]:::black
	b4["b"]:::red
    a4["c"]:::black

    b4 --> a4
    b4 --> c4
end

first -->|insert b| second --> |rotate| third --> |rotate| fourth

classDef red 1,fill:#FF4D4D,stroke:#333,background-color:#ff4d4d;
classDef black fill:#000,stroke:none,stroke-width:2px,color:#fff;
classDef leaf fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5;
```

### rotate

#### left rotate

```mermaid
flowchart LR

subgraph before
	direction TB

	E1["E"]:::black
	S1["S"]:::red
	Small{"(-∞, E)"}:::leaf
	Medium{"(E, S)"}:::leaf
	Large{"(S, +∞)"}:::leaf

	E1 --> Small
	E1 --> S1

	S1 --> Medium
	S1 --> Large
end

subgraph after
	direction TB

	E2["E"]:::red
	S2["S"]:::black
	Small2{"(-∞, E)"}:::leaf
	Medium2{"(E, S)"}:::leaf
	Large2{"(S, +∞)"}:::leaf

	S2 --> E2
	S2 --> Large2

	E2 --> Small2
	E2--> Medium2
end

before -->|left rotate| after

classDef red 1,fill:#FF4D4D,stroke:#333,background-color:#ff4d4d;
classDef black fill:#000,stroke:none,stroke-width:2px,color:#fff;
classDef leaf fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5;
```

#### right rotate

```mermaid
flowchart LR

subgraph before
	direction TB

	S1["S"]:::black
	E1["E"]:::red
	Small{"(-∞, E)"}:::leaf
	Medium{"(E, S)"}:::leaf
	Large{"(S, +∞)"}:::leaf

	S1 --> E1
	S1 --> Large

	E1 --> Small
	E1 --> Medium

end

subgraph after
	direction TB

	E2["E"]:::black
	S2["S"]:::red
	Small2{"(-∞, E)"}:::leaf
	Medium2{"(E, S)"}:::leaf
	Large2{"(S, +∞)"}:::leaf

	E2 --> Small2
	E2 --> S2

	S2 --> Medium2
	S2 --> Large2

end

before -->|left rotate| after

classDef red 1,fill:#FF4D4D,stroke:#333,background-color:#ff4d4d;
classDef black fill:#000,stroke:none,stroke-width:2px,color:#fff;
classDef leaf fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5;
```



## AVL

### left rotation

```mermaid
---
title: a imbalanced tree
---
flowchart TB

4 --> 3:::child
4 --> 5

3 --> 1
3 --> Nil1["Nil"]:::grandchild
1 --> 0
1 --> NIl2["Nil"]

classDef child 1,fill:#FFCCCC,stroke:#333;
classDef grandchild fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5
```

```mermaid
---
title: first style of rotation
---
flowchart TB

4 --> 1
4 --> 5

1 --> 0
1 --> 3

classDef child 1,fill:#FFCCCC,stroke:#333;
classDef grandchild fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5
```

```mermaid
---
title: second imbalanced tree
---
flowchart TB

3 --> 1

1 --> 0
3 --> 4
4 --> 5

classDef child 1,fill:#FFCCCC,stroke:#333;
classDef grandchild fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5
```

### the rotation of each different situations

> The other situation is just converse to below situation.

```mermaid
---
title: left rotation
---
flowchart TB

4 --> 3:::child
4 --> 5

3 --> 1
3 --> Nil1["Nil"]:::grandchild
1 --> 0
1 --> NIl2["Nil"]:::grandchild

classDef child 1,fill:#FFCCCC,stroke:#333;
classDef grandchild fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5
```

```mermaid
---
title: right rotation then left rotation
---
flowchart TB

4 --> 3:::child
4 --> 5

3 --> 0
3 --> Nil1["Nil"]:::grandchild
0 --> NIl2["Nil"]:::grandchild
0 --> 1

classDef child 1,fill:#FFCCCC,stroke:#333;
classDef grandchild fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5
```

### how to choose strategy

| factor of imbalanced node | factor of imbalanced node's child node | strategy                          |
| ------------------------- | -------------------------------------- | --------------------------------- |
| > 1                       | > 0                                    | right rotation                    |
| > 1                       | < 0                                    | left rotation then right rotation |
| < 1                       | > 0                                    | right rotation then left rotation |
| < 1                       | < 0                                    | left rotation                     |



