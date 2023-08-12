# Readme.md

## Project Overview

This Rust project reads in a CSV file containing trade data, and sorts the data based on the cumulative value field. Two sorting algorithms are provided: Heap Sort and Quick Sort. After sorting, the program prints the sorted data and the time taken to sort the data. 

## Code Structure

The code is divided into several functions:

- `heapify`: Helper function used in Heap Sort. This function maintains the heap property, i.e., for any given node i, the value of i is not smaller than the values of its children.
- `heap_sort`: Implementation of Heap Sort algorithm.
- `partition`: Helper function used in Quick Sort. This function takes the last element as a pivot, places the pivot at its correct position, and places all smaller elements to the left of the pivot and all larger elements to the right of the pivot.
- `quick_sort`: Implementation of Quick Sort algorithm.
- `read_data`: Reads a CSV file and returns a vector of `Data` structs.
- `print_data`: Prints the data.
- `save_to_file`: Writes the sorted data to a file.
- `surround_with_quotes_if_comma`: Returns a string surrounded by quotes if the string contains a comma; otherwise, returns the string itself. This function is useful when writing CSV data, to ensure that fields containing commas are correctly interpreted.
- `user_input`: Reads a line from the standard input and returns it as a string.
- `main`: Reads the CSV data, prints the number of records, prompts the user to choose a sorting algorithm, sorts the data using the chosen algorithm, and prints the sorted data and the time taken to sort the data.

## Performance Tweaks

The program has implemented a few performance tweaks to optimize the sorting process:

1. **In-place Sorting**: Both Quick Sort and Heap Sort are performed in-place, i.e., they sort the data within the array itself, without requiring any extra space. This significantly reduces memory usage.

2. **Efficient Data Structures**: The data is stored in a vector of structs, which provides efficient random access. Also, by defining the struct `Data`, we are able to group related data together, which makes the code more readable and maintainable.

3. **Rust's Ownership Model**: Rust's ownership model guarantees memory safety without needing a garbage collector. This results in efficient memory usage and helps avoid common programming errors like null pointer dereferencing, dangling pointers, etc.

4. **Avoiding Repeated Computation**: In both the `heapify` and `partition` functions, we calculate the index of the left child, right child, and parent only once and reuse these values, instead of calculating them every time they are needed.

5. **Performance Measurement**: The `SystemTime` class is used to measure the time taken by the sorting algorithms. This helps in identifying bottlenecks and assessing the performance of the code.