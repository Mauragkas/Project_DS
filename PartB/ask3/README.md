# Hash table-based data management system

This is a simple data management system implemented in Rust. It allows users to search, edit, and delete data records stored in a CSV file.

## Code Overview

The code consists of the following main components:

1. `Data` struct: Represents a single data record with various fields such as direction, year, date, weekday, country, commodity, transport mode, measure, value, and cumulative.

2. `Node` struct: Represents a node in a linked list. Each node contains a `Data` object and a reference to the next node.

3. `LinkedList` struct: Represents a linked list data structure. It consists of a pointer to the first node and a pointer to the last node.

4. Hash table: Implemented as a vector of linked lists. The hash table is used to store the data records based on their date. The `hash` function calculates the hash value for a given date, which is used to determine the index of the linked list in the vector.

5. Various functions: The code provides functions to initialize the hash table, insert data into the hash table, search for data records, edit data records, delete data records, and read data from a CSV file.

6. Main function: Implements the user interface for interacting with the data management system. It allows users to perform operations like searching, editing, and deleting data records.

## Performance Tweaks

To improve performance, the following optimizations have been made:

1. Linked List Implementation: The data records are stored in a linked list data structure instead of using a dynamic array. This allows efficient insertion and deletion operations as the linked list can easily accommodate changes to the data structure.

2. Hash Table: The data records are stored in a hash table based on their date. This provides efficient searching, editing, and deleting operations with an average time complexity of O(1) for each operation.

3. Hash Function: The `hash` function calculates the hash value for a given date by summing up the ASCII values of its characters and taking the modulo of the sum with a fixed constant (`MOD`). This ensures a relatively uniform distribution of data records across the hash table, minimizing collisions and improving performance.

4. Separate Chaining: Collisions in the hash table are handled using separate chaining. If multiple data records have the same hash value, they are stored in the same linked list at the corresponding index of the hash table. This allows efficient traversal of data records with the same hash value during search, edit, and delete operations.

5. CSV Reading: The `read_data` function uses the `csv` crate to efficiently read data records from a CSV file. It parses each record and inserts it into the hash table, minimizing the time and memory overhead of reading and processing large CSV files.

## Conclusion

This data management system provides a basic framework for storing, searching, editing, and deleting data records. The use of a hash table with separate chaining ensures efficient performance for various operations. The code can be extended and customized to meet specific requirements or integrated into larger systems.

Feel free to explore and modify the code according to

 your needs.