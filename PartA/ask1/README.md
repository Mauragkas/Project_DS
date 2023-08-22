# Readme.md

## Description

This project is a command-line program written in Rust that reads data from a CSV file into a `Vec<Data>` where `Data` is a custom struct. It sorts the data using either a counting sort or merge sort algorithm. The user can choose which algorithm to use at runtime.

The CSV file contains trade data with several fields such as `direction`, `year`, `date`, `weekday`, `country`, `commodity`, `transport_mode`, `measure`, `value`, and `cumulative`. These fields are reflected in the `Data` struct.

The program can print the sorted data to the console.

## Performance Improvements

### Counting Sort

The counting sort algorithm sorts data by value and then by date. It uses a custom comparison function that takes into account both the value and the date. This ensures a stable sort with the correct ordering of data records.

Counting sort works by creating a count vector with a size equal to the range of the input data (max - min + 1). Each element in the count vector represents the number of occurrences of a specific value. Then, it populates a sorted array using the count vector.

This algorithm is efficient for sorting data with a small range of values and a large number of records. However, it may not be suitable for data with a wide range of values.

### Merge Sort

The merge sort algorithm was implemented using Rust's rayon crate for parallelism. The program divides the data into smaller chunks, sorts each chunk in parallel, and then merges the sorted chunks back together.

Merge sort has a time complexity of O(n log n) and is an efficient sorting algorithm for large datasets. Its parallel implementation further improves its performance by utilizing multiple CPU cores.

## Usage

1. Run the program.
2. Select the sorting algorithm: enter `1` for counting sort or `2` for merge sort.
3. The sorted data will be printed to the console, along with the time taken by the sorting operation.

The program includes the option to save the sorted data to a CSV file, but this feature is currently disabled (`#[allow(unused)]`). To enable it, remove this attribute and call the `save_to_file` function with the sorted data and the desired filename.

---