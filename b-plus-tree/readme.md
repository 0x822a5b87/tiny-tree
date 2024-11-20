# readme

> [Here is another explanation of B+Tree in java version](./java-code-with-explanation.md)

## Performance Optimization Plan

- Using a red-black tree to replace the array inside the B+Tree in order to decrease the performance of insert and delete operations.
- Using `[]byte` to replace the `slice` inside in order to save the memory usage.
- fix severe BUG : **leaf nodes could become invalid under certain special conditions.**
