# AVL Tree Data Structure

This code implements an AVL (Adelson-Velsky Landis) tree data structure in Rust. An AVL tree is a self-balancing binary search tree where the heights of the two child subtrees of any node differ by at most one. This balancing property ensures that the tree remains balanced, which leads to efficient search, insertion, and deletion operations.

## Code Explanation

The code consists of the following main components:

1. `Data` struct: Represents the data stored in each tree node. It contains various fields like `direction`, `year`, `date`, `weekday`, `country`, `comodity`, `transport_mode`, `measure`, `value`, and `cumulative`.

2. `Node` struct: Represents a node in the AVL tree. It contains a shared reference to `Data`, as well as optional left and right child nodes, and a height value to track the balance factor.

3. `AvlTree` struct: Represents the AVL tree itself. It contains an optional root node.

4. Tree operations:
   - `insert`: Inserts a new node into the tree while maintaining balance.
   - `rotate_left` and `rotate_right`: Perform rotation operations to balance the tree.
   - `balance`: Balances the tree by updating heights and performing rotations if necessary.
   - `node_with_min_value` and `node_with_max_value`: Retrieve nodes with the minimum and maximum values in the tree.
   - `nodes_with_same_value`: Finds nodes with the same value as the given value.

5. File reading and user input:
   - `read_data`: Reads data from a CSV file and constructs an AVL tree.
   - `user_input`: Reads user input from the command line.
   - `print_data`: Prints the fields of a `Data` struct.

6. Main function: Reads data from a CSV file, constructs an AVL tree, and provides a menu-based interface for performing operations like finding nodes with the maximum and minimum values.

## Performance Tweaks

To improve performance, the following optimizations have been made:

1. **AVL Tree**: The AVL tree self-balances to maintain logarithmic time complexity for search, insert, and delete operations. This ensures that the tree remains balanced and the height of the tree is minimized.

2. **Balancing Operations**: The code implements efficient rotation operations (`rotate_left` and `rotate_right`) to balance the tree. These operations are performed when necessary to maintain the balance factor of the nodes.

3. **Shared Ownership**: The `Data` struct is wrapped in an `Rc` (Reference Counted) smart pointer to allow multiple nodes to reference the same data without cloning it. This reduces memory usage and improves performance when inserting and manipulating nodes.

4. **Early Exit on Errors**: When reading the CSV file, the code immediately exits the program if there is an error reading a record or the file itself. This prevents unnecessary processing and error propagation.

5. **Efficient Searching**: The code provides functions (`node_with_min_value`, `node_with_max_value`, `nodes_with_same_value`) to efficiently find nodes with specific values in the AVL tree. These functions traverse the tree only as necessary and return early when the desired nodes are found.

6. **Early Printing and Limiting Results**: When finding nodes with the maximum or minimum values, the code limits the output to the first 10 nodes and stops further traversal. This avoids printing an excessive number of results and improves overall performance.

## Conclusion

The provided code demonstrates the implementation of an AVL tree data structure in Rust. By using self-balancing techniques and efficient algorithms, the AVL tree ensures optimal performance for search, insertion, and deletion operations. The code also incorporates various optimizations to improve memory usage and execution speed. With the help of the menu-based interface, users can interact with the AVL tree and perform operations on the stored data efficiently.