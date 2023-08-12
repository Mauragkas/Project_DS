#![allow(unused_imports)]
use std::fs::File;
use std::io::{Write, BufRead, BufReader};
use std::time::SystemTime;
use std::path::Path;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::cmp;

mod tests;

#[derive(Debug, Clone)]
struct Data {
    direction: String,
    year: u16,
    date: String,
    weekday: String,
    country: String,
    comodity: String,
    transport_mode: String,
    measure: String,
    value: u64,
    cumulative: u64,
}

impl Data {
    fn new() -> Data {
        Data {
            direction: String::new(),
            year: 0,
            date: String::new(),
            weekday: String::new(),
            country: String::new(),
            comodity: String::new(),
            transport_mode: String::new(),
            measure: String::new(),
            value: 0,
            cumulative: 0,
        }
    }
}

/*
by using concurent programming, we can make the counting sort faster by about 33% (~6s to ~4s)
but it is still slower than merge sort 
*/
async fn counting_sort(data: &mut Vec<Data>) {
    let min_value = data.iter().map(|d| d.value).min().unwrap() as usize;
    let max_value = data.iter().map(|d| d.value).max().unwrap() as usize;

    // `Arc` and `Mutex` are used to safely share and mutate `count_vec` across multiple async tasks.
    let count_vec = Arc::new(Mutex::new(vec![0; max_value - min_value + 1]));

    let mut handles = vec![];
    for d in data.iter() {
        let count_vec = Arc::clone(&count_vec);
        let value = d.value as usize - min_value;
        let handle = tokio::spawn(async move {
            let mut count_vec = count_vec.lock().await;
            count_vec[value] += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }

    let start = SystemTime::now();
    let mut count_vec = count_vec.lock().await;
    let mut sum = 0;
    for i in count_vec.iter_mut() {
        *i += sum;
        sum = *i;
    }
    println!("time to calculate cumulative values: {:?}ms", start.elapsed().unwrap().as_millis());

    let mut sorted_data = vec![Data::new(); data.len()];
    for d in data.iter().rev() {
        let index = d.value as usize - min_value;
        sorted_data[count_vec[index] - 1] = d.clone();
        count_vec[index] -= 1;
    }
    *data = sorted_data;
}

fn merge(left_vec: &[Data], right_vec: &[Data]) -> Vec<Data> {
    let mut merged_vec = Vec::with_capacity(left_vec.len() + right_vec.len());
    let mut left_iter = left_vec.iter().peekable();
    let mut right_iter = right_vec.iter().peekable();

    while let (Some(left), Some(right)) = (left_iter.peek(), right_iter.peek()) {
        if left.value < right.value {
            merged_vec.push(left_iter.next().unwrap().clone());
        } else {
            merged_vec.push(right_iter.next().unwrap().clone());
        }
    }
    merged_vec.extend(left_iter.cloned());
    merged_vec.extend(right_iter.cloned());

    merged_vec
}

fn merge_sort(data: &[Data]) -> Vec<Data> {
    if data.len() <= 1 {
        return data.to_vec();
    }
    let mid = data.len() / 2;
    let left_vec = merge_sort(&data[0..mid]);
    let right_vec = merge_sort(&data[mid..]);
    merge(&left_vec, &right_vec)
}

fn read_data(filename: &str) -> Vec<Data> {
    let mut reader = csv::Reader::from_path(filename).unwrap();
    let mut data = Vec::new();

    for result in reader.records() {
        let record = result.unwrap();
        let direction = record[0].to_string();
        let year = record[1].parse::<u16>().unwrap();
        let date = record[2].to_string();
        let weekday = record[3].to_string();
        let country = record[4].to_string();
        let comodity = record[5].to_string();
        let transport_mode = record[6].to_string();
        let measure = record[7].to_string();
        let value = record[8].parse::<u64>().unwrap();
        let cumulative = record[9].parse::<u64>().unwrap();

        data.push(
            Data {
                direction,
                year,
                date,
                weekday,
                country,
                comodity,
                transport_mode,
                measure,
                value,
                cumulative,
            });
    }

    return data;
}

#[allow(dead_code)]
fn print_data(data: &Vec<Data>) {
    for d in data {
        println!("{} {} {} {} {} {} {} {} {} {}",
            d.direction,
            d.year,
            d.date,
            d.weekday,
            d.country,
            d.comodity,
            d.transport_mode,
            d.measure,
            d.value,
            d.cumulative
        );
    }

    println!("--------------------------------");
    println!("{} records", data.len());
    println!("--------------------------------");
}

#[allow(dead_code)]
fn save_to_file(data: &Vec<Data>, filename: &str) {
    let mut file = File::create(filename).expect("Unable to create file");
    writeln!(file, "Direction,Year,Date,Weekday,Country,Commodity,Transport_Mode,Measure,Value,Cumulative").expect("Unable to write header");
    for d in data {
        writeln!(file, "{},{},{},{},{},{},{},{},{},{}",
            surround_with_quotes_if_comma(d.direction.as_str()),
            d.year,
            surround_with_quotes_if_comma(d.date.as_str()),
            surround_with_quotes_if_comma(d.weekday.as_str()),
            surround_with_quotes_if_comma(d.country.as_str()),
            surround_with_quotes_if_comma(d.comodity.as_str()),
            surround_with_quotes_if_comma(d.transport_mode.as_str()),
            surround_with_quotes_if_comma(d.measure.as_str()),
            d.value,
            d.cumulative
        ).expect("Unable to write data");
    }
}

fn surround_with_quotes_if_comma(string: &str) -> String {
    if string.contains(",") {
        format!("\"{}\"", string)
    } else {
        String::from(string)
    }
}

fn user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    input
}

#[tokio::main]
async fn main() {
    let mut data_vector = read_data("effects.csv");
    
    println!("Select sorting algorithm:");
    println!("1. Counting sort");
    println!("2. Merge sort");
    print!("Enter your choice: ");
    std::io::stdout().flush().unwrap();

    let choice = user_input();

    match choice.as_str() {
        "1" => {
            let start = SystemTime::now();
            counting_sort(&mut data_vector).await;
            let end = SystemTime::now();
            print_data(&data_vector);
            println!("Counting sort took {} ms", end.duration_since(start).unwrap().as_millis());
            // println!("Counting sort took {} ns", end.duration_since(start).unwrap().as_nanos());

        },
        "2" => {
            let start = SystemTime::now();
            let sorted_data = merge_sort(&data_vector);
            let end = SystemTime::now();
            print_data(&sorted_data);
            println!("Merge sort took {} ms", end.duration_since(start).unwrap().as_millis());
        },
        _ => {
            println!("Invalid choice");
        }
    }
}
