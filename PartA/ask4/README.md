# Binary Search Optimization

This repository contains a Rust program that performs a binary search on a dataset and finds a specific date. Several performance optimizations have been applied to improve the program's efficiency.

## Code Explanation

The code consists of the following components:

1. `Data` struct: Represents the structure of the data read from the CSV file. It contains various fields such as direction, year, date, weekday, country, commodity, transport mode, measure, value, and cumulative.

2. `date_to_days` function: Converts a date string in the format "dd/mm/yyyy" to the number of days since year 0. This conversion allows for easier comparison and searching based on dates.

3. `read_data` function: Reads data from a CSV file and populates a vector of `Data` structs.

4. `user_input` function: Prompts the user to enter a date and returns the input as a string.

5. `in_range` function: Checks if a given date falls within the range of dates in the dataset.

6. `bis` function: Performs the binary search on the dataset to find the specified date. It utilizes an optimized binary search algorithm that handles edge cases and performs efficient searching.

7. `main` function: The entry point of the program. It reads the data from a CSV file, prompts the user for a date, performs the binary search, and outputs the results.

## Performance Optimizations

The following performance optimizations have been made to improve the efficiency of the binary search algorithm:

1. **Binary Search**: The core search algorithm uses a binary search technique to locate the desired date (more in the report). Binary search has a time complexity of O(log n), making it efficient for large datasets.

2. **Early Exit**: If the user enters a date that is outside the range of dates in the dataset, the program terminates early, avoiding unnecessary computations.

3. **Optimized Binary Search**: The `bis` function applies an optimized binary search algorithm. It uses interpolation to estimate the position of the target date within the dataset and adjusts the search range accordingly. This optimization reduces the number of iterations required to find the date, resulting in improved performance.

4. **Early Termination**: Once the target date is found, the program immediately outputs the result and terminates, minimizing any additional processing.

## Conclusion

The optimized binary search algorithm implemented in this program improves the efficiency of searching for specific dates within a large dataset. By applying early termination and interpolation techniques, the program achieves better performance, reducing the search time and improving the overall user experience. Feel free to modify and adapt the code as per your requirements.