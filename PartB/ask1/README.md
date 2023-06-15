# AVL Tree Data Structure

This code implements an AVL tree data structure in Rust. The AVL tree is a self-balancing binary search tree that maintains its balance factor, ensuring efficient insertion, deletion, and searching operations.

## Code Explanation

The code consists of several parts:

1. The `Data` struct represents the data stored in each node of the AVL tree. It contains various fields such as direction, year, date, weekday, country, commodity, transport mode, measure, value, and cumulative.

2. The `Node` struct represents a node in the AVL tree. It contains the `Data` object, left and right child pointers, and a height field to track the balance factor.

3. The `AvlTree` struct represents the AVL tree itself. It maintains a root node and provides methods for insertion, deletion, searching, and editing nodes.

4. The code includes utility functions for calculating the height, balance factor, and updating the height of nodes in the tree.

5. The AVL tree operations, such as insertion and deletion, are implemented using recursive functions that maintain the balance factor of nodes and perform rotations if necessary to rebalance the tree.

6. The code also includes functions for reading data from a CSV file, printing data, and taking user input for interacting with the AVL tree.

7. The main function initializes an AVL tree, reads data from a CSV file, and provides a menu-driven interface for performing operations on the tree.

## Performance Tweaks

To improve the performance of the AVL tree implementation, the following tweaks have been made:

1. **Balancing**: The AVL tree uses rotations to balance the tree after insertions and deletions. These rotations ensure that the balance factor of each node is within the acceptable range (-1, 0, 1). By maintaining balanced trees, the AVL tree provides efficient search, insert, and delete operations with a worst-case time complexity of O(log n).

2. **Height Optimization**: The height of each node is calculated and updated during tree operations. This optimization allows for quick determination of the balance factor and avoids recalculating the heights of the entire subtree. By updating the height of nodes incrementally, the AVL tree operations achieve better performance.

3. **Data Structure Choice**: The AVL tree data structure is chosen for its self-balancing property, which ensures the tree's height is logarithmic and provides efficient search, insert, and delete operations. This choice enables the implementation to handle large datasets efficiently.

4. **Input Validation**: The code includes input validation to handle invalid user input, such as incorrect date formats or non-numeric values. This validation prevents errors and ensures the stability and correctness of the program.

5. **CSV Reading Optimization**: The code uses the `csv` crate to read data from a CSV file. The use of a dedicated library for CSV parsing ensures efficient and reliable data reading, improving the overall performance of the program.

6. **Early Exit on Error**: The code includes error handling for file reading and parsing operations. In case of an error, the program exits with an appropriate error message. This early exit prevents unnecessary computations on invalid data, improving performance and avoiding potential issues.

## Conclusion

The provided code implements an AVL tree data structure in Rust, providing efficient search, insert, delete, and edit operations on the stored data. The performance tweaks made, such as self-balancing, height optimization, and input validation, ensure that the AVL tree operates efficiently on large datasets. The code also includes CSV reading optimizations and error handling to improve overall performance and stability.