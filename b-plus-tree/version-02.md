# another version

## Node

```java
public class Node {
    InternalNode parent;
}
```

## Pair

```java
public class Pair implements Comparable<Pair> {
    int    key;
    double value;

    public Pair(int key, double value) {
        this.key   = key;
        this.value = value;
    }

    public int compareTo(Pair o) {
        return Integer.compare(key, o.key);
    }
}
```

