# Readme.md

## Description

This project is a command-line program written in Rust that reads data from a CSV file into a `Vec<Data>` where `Data` is a custom struct. It then sorts the data using either a counting sort or merge sort algorithm. The user can choose which algorithm to use. 

The CSV file contains trade data with several fields such as `direction`, `year`, `date`, `weekday`, `country`, `commodity`, `transport_mode`, `measure`, `value`, and `cumulative`. These fields are reflected in the `Data` struct.

The program offers the functionality to print the sorted data to the console or to save it to a new CSV file.

## Performance Improvements

### Concurrency in Counting Sort

The counting sort implementation was enhanced by leveraging Rust's async programming features to achieve concurrency. This allows multiple elements of the data to be processed simultaneously.

This concurrency is achieved by spinning up asynchronous tasks that process different parts of the count vector in parallel, each of which increases the count of a specific value. To ensure safe concurrent access to the count vector, it is wrapped in an `Arc<Mutex<_>>`. 

The `Arc` (Atomic Reference Count) ensures that the count vector can be safely shared across multiple tasks, and the `Mutex` (Mutual Exclusion) ensures that only one task can mutate the count vector at any given time.

With this concurrency improvement, the counting sort algorithm was found to be about 33% faster, reducing the time from ~6s to ~4s on the tested dataset.

However, it's important to note that counting sort's efficiency highly depends on the range of the input data. It performs best when the difference between the maximum and minimum values is not significantly greater than the number of input data.

### Merge Sort

In addition to counting sort, this program offers a merge sort algorithm for sorting the data. While merge sort doesn't use concurrency, it's still efficient for larger data sets or data sets with a larger range of values. Merge sort has a time complexity of O(n log n), which can be faster than counting sort in some cases (not this time, though).

## Usage

1. Select the sorting algorithm: enter `1` for counting sort or `2` for merge sort.
2. The sorted data will be printed to the console, along with the time taken by the sorting operation.

The program also includes the option to save the sorted data to a CSV file, but this feature is currently disabled (`#[allow(dead_code)]`). To enable it, simply remove this attribute and call the `save_to_file` function with the sorted data and the desired filename.