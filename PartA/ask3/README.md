# README.md

## Introduction

This Rust project is a data search tool that reads a CSV file containing trade data, sorts it by date, and allows users to quickly search for data on a specific date. The program employs two algorithms for the search operation: a binary search and an interpolation search. The primary goal of this program is to offer rapid data access and retrieval, even with a large dataset.

The data structure used to store the data is a vector of `Data` struct instances, where `Data` is a custom data type that represents each record in the CSV file. Each `Data` instance holds several fields, including `direction`, `year`, `date`, `weekday`, `country`, `comodity`, `transport_mode`, `measure`, `value`, and `cumulative`.

## Performance Tweaks

### Date Conversion to Days

This program optimizes the search operation by converting the dates into a numerical representation (number of days from the year 0). This conversion significantly improves the performance by simplifying the date comparison during the search operation. Instead of comparing dates (strings), we are comparing integers, which is a much faster operation.

### Binary Search

Binary search is an efficient search algorithm for finding an item from a sorted list of items. It works by repeatedly dividing in half the portion of the list that could contain the item, until you've narrowed down the possible locations to just one.

### Interpolation Search

Interpolation search is an algorithm for searching for a key in an array that has been ordered by numerical values assigned to the keys (probabilistic search). It improves upon binary search, where the key value being searched is used to calculate an estimate of its position in the array, by using the key values of the first and last elements in the search sub-array, along with the length of the sub-array.

### Parallel Processing

While not implemented in the current version of the code, parallel processing would be an excellent way to further improve the program's performance. This could involve dividing the dataset into multiple segments and processing them concurrently (tryed with `rayon` and `tokio` crates but it was slower than the sequential version).

## Usage

1. Ensure that you have Rust installed on your machine.
2. Run the program by entering `cargo run` in the command line.

You will be prompted to input a date in the `dd/mm/yyyy` format. If the date exists in the dataset, the program will output the corresponding record.

## Conclusion

In conclusion, while this program demonstrates the use of performance tweaks like date conversion and efficient search algorithms, it's important to note that parallel processing, in its current implementation, incurred a high cost and resulted in lower performance compared to sequential execution. 

Additionally, it's crucial to recognize that the performance of any program is primarily determined by the characteristics of the data being processed (mostly for the interpolation seach). Understanding the data's nature and structure is vital for selecting suitable algorithms and optimizations.