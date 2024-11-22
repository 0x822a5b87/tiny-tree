# source

>Here is the source code with comment in Java

```java
package util;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Bpt {

    // Node creation
    static class Node {
        int order;

        // There is a particular point that can be easily misunderstood: the handling differs when the node is a leaf node or a non-leaf node.
        // For a non-leaf node:
        //  - keys   are used for locating downstream nodes, where keys.len() == values.len() - 1 && value[i].len() == 1
        //  - values represent the values of the child nodes of the current node
        // For a leaf node:
        //  - keys   represent the keys in their corresponding key-value pairs, where keys.len() == values.len()
        //  - values represent the values in their corresponding key-value pairs, which can be multiple, where values[i].len() == count(key[i])
        // Here, count(key[i]) represents the number of times the key has been inserted. For example, if we insert two values with key==15, then count(15) == 2.
        List<Integer>    indices;
        // the value in values is a list of nodes because the B-Plus-Tree allows the insertion of duplicate values
        List<List<Node>> values;
        Node             nextKey;
        Node             parent;
        boolean          isLeaf;

        // Node constructor
        public Node(int order) {
            this.order   = order;
            this.indices = new ArrayList<>();
            this.values  = new ArrayList<>();
            this.nextKey = null;
            this.parent  = null;
            this.isLeaf  = false;
        }

        // Insert at the leaf
        public void insertAtLeaf(Integer value, Node key) {
            if (!this.indices.isEmpty()) {
                for (int i = 0; i < this.indices.size(); i++) {
                    if (value.equals(this.indices.get(i))) {
                        // value == keys[i]
                        // values contain current key-value pair, just add it to corresponding array
                        this.values.get(i).add(key);
                        break;
                    } else if (value.compareTo(this.indices.get(i)) < 0) {
                        // find the position where the new value should be placed and insert value.
                        this.indices.add(i, value);
                        this.values.add(i, new ArrayList<>());
                        this.values.get(i).add(key);
                        break;
                    } else if (i + 1 == this.indices.size()) {
                        // new value is the largest value in the array, put it at the end of the array.
                        this.indices.add(value);
                        this.values.add(new ArrayList<>());
                        this.values.get(i + 1).add(key);
                        break;
                    }
                }
            } else {
                this.indices.add(value);
                this.values.add(new ArrayList<>());
                this.values.get(0).add(key);
            }
        }
    }

    // B plus tree
    static class BPlusTree {
        Node root;

        // B plus tree constructor
        public BPlusTree(int order) {
            this.root        = new Node(order);
            this.root.isLeaf = true;
        }

        // Insert operation
        public void insert(Integer value, Node key) {
            Node oldNode = this.search(value);
            oldNode.insertAtLeaf(value, key);

            if (oldNode.indices.size() == oldNode.order) {
                // exceed the maximum size of current node for current order, must divide into separate node

                // create new node
                // 1. new node is still leaf node;
                // 2. new node share the same parent node with old node;
                // 3. new node contains the rear half elements of the old node;
                // 4. new node's next node is old node's next node;
                Node newNode = new Node(oldNode.order);
                newNode.isLeaf = true;
                newNode.parent = oldNode.parent;
                int mid = (int) Math.ceil(oldNode.order / 2.0) - 1;
                newNode.indices = new ArrayList<>(oldNode.indices.subList(mid + 1, oldNode.indices.size()));
                newNode.values  = new ArrayList<>(oldNode.values.subList(mid + 1, oldNode.values.size()));
                newNode.nextKey = oldNode.nextKey;

                // delete the elements that were moved to new node
                oldNode.indices = new ArrayList<>(oldNode.indices.subList(0, mid + 1));
                oldNode.values  = new ArrayList<>(oldNode.values.subList(0, mid + 1));
                oldNode.nextKey = newNode;

                // node's parent must change as well
                // the configuration of parent node is as follows:
                //
                //      leftPtr index rightPtr
                //       /                 \
                //      /                   \
                // oldNode                 newNode
                //
                // index is the min size of new node, this is really important because it determines how we search target value
                // in the structure.
                this.insertInParent(oldNode, newNode.indices.get(0), newNode);
            }
        }

        // Search operation for different operations
        public Node search(Integer value) {
            Node currentNode = this.root;
            while (!currentNode.isLeaf) {
                for (int i = 0; i < currentNode.indices.size(); i++) {
                    if (value.equals(currentNode.indices.get(i))) {
                        // value == keys[i]
                        // search right
                        currentNode = currentNode.values.get(i + 1).get(0);
                        break;
                    } else if (value.compareTo(currentNode.indices.get(i)) < 0) {
                        // value < keys[i]
                        // search left
                        currentNode = currentNode.values.get(i).get(0);
                        break;
                    } else if (i + 1 == currentNode.indices.size()) {
                        // at the end of keys
                        // search right
                        currentNode = currentNode.values.get(i + 1).get(0);
                        break;
                    }
                }
            }
            return currentNode;
        }

        // Find the node
        public boolean find(Integer value, Node key) {
            Node leaf = this.search(value);
            for (int i = 0; i < leaf.indices.size(); i++) {
                if (leaf.indices.get(i).equals(value)) {
                    if (leaf.values.get(i).contains(key)) {
                        return true;
                    } else {
                        return false;
                    }
                }
            }
            return false;
        }

        // Inserting at the parent
        public void insertInParent(Node n, Integer value, Node ndash) {
            if (this.root == n) {
                // the current separation is being conducted on the root node, so it must update the root node in comparison to other operations.

                // create new root node
                // We should notice that the separation of a root node only results in two child nodes, regardless of the original configuration of the root node
                Node rootNode = new Node(n.order);
                rootNode.indices.add(value);
                rootNode.values.add(new ArrayList<>());
                rootNode.values.add(new ArrayList<>());
                rootNode.values.get(0).add(n);
                rootNode.values.get(1).add(ndash);

                this.root    = rootNode;
                n.parent     = rootNode;
                ndash.parent = rootNode;
                return;
            }

            Node parentNode = n.parent;
            for (int i = 0; i < parentNode.values.size(); i++) {
                if (parentNode.values.get(i).get(0) == n) {
                    // find the position of n in the parent node

                    // insert new index and new values
                    parentNode.indices.add(i, value);
                    List<Node> right = Collections.singletonList(ndash);
                    parentNode.values.add(i + 1, right);

                    if (parentNode.values.size() > parentNode.order) {
                        // divide parent node as well

                        // local variable oldNode is redundant, keep it just for the readability of the code
                        Node oldNode = parentNode;

                        // the logic is almost the same as dividing a leaf node except these three procedures:
                        // 1. the new node is not leaf node;
                        // 2. a subtle difference between leaf node and non-leaf node when processing the index of indices
                        // 3. leaf nodes must set their next key to point to the next leaf in order of facilitate range query operation
                        // 4. update node's parent
                        Node newParentDash = new Node(oldNode.order);
                        // this statement is actually redundant, keep it just for clarification
                        newParentDash.isLeaf = false;
                        newParentDash.parent = oldNode.parent;
                        int mid = (int) Math.ceil(oldNode.order / 2.0) - 1;
                        newParentDash.indices = new ArrayList<>(
                                oldNode.indices.subList(mid + 1, oldNode.indices.size()));
                        newParentDash.values  = new ArrayList<>(
                                oldNode.values.subList(mid + 1, oldNode.values.size()));

                        oldNode.indices = new ArrayList<>(oldNode.indices.subList(0, mid + 1));
                        oldNode.values = new ArrayList<>(oldNode.values.subList(0, mid + 1));

                        // update node's parent
                        for (int j = 0; j < oldNode.values.size(); j++) {
                            oldNode.values.get(j).get(0).parent = oldNode;
                        }
                        for (int j = 0; j < newParentDash.values.size(); j++) {
                            newParentDash.values.get(j).get(0).parent = newParentDash;
                        }

                        Integer indexValue = newParentDash.indices.get(0);
                        this.insertInParent(oldNode, indexValue, newParentDash);
                    }
                    break;
                }
            }
        }
    }

    public static void main(String[] args) {

        int order = 5;

        BPlusTree bplusTree = new BPlusTree(order);

        for (int i = 0; i < 100; i++) {
            Node node = new Node(order);
            bplusTree.insert(i, node);

            assert bplusTree.find(i, node);
        }

        printTree(bplusTree);
    }

    // Print the tree
    public static void printTree(BPlusTree tree) {
        List<Node> lst = new ArrayList<>();
        lst.add(tree.root);
        List<Integer> level = new ArrayList<>();
        level.add(0);
        Node leaf     = null;
        int  flag     = 0;
        int  lev_leaf = 0;

        while (!lst.isEmpty()) {
            Node x   = lst.remove(0);
            int  lev = level.remove(0);
            if (!x.isLeaf) {
                for (int i = 0; i < x.values.size(); i++) {
                    System.out.println(x.values.get(i).get(0).indices);
                }
            } else {
                for (int i = 0; i < x.values.size(); i++) {
                    System.out.println(x.values.get(i).get(0).indices);
                }
                if (flag == 0) {
                    lev_leaf = lev;
                    leaf     = x;
                    flag     = 1;
                }
            }
        }
    }
}

```
