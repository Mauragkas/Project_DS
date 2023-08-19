#![allow(unused)]
use std::fs::File;
use std::io::{Write, BufRead, BufReader};
use std::time::SystemTime;
use std::path::Path;
use std::sync::Arc;
use std::cmp;

use csv::{ReaderBuilder, ByteRecord};

use rayon::prelude::*;

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

fn counting_sort(data: &mut Vec<Data>) {
    let min_value = data.iter().map(|d| d.value).min().unwrap() as usize;
    let max_value = data.iter().map(|d| d.value).max().unwrap() as usize;

    let mut count_vec = vec![0; max_value - min_value + 1];
    for d in data.iter() {
        let value = d.value as usize - min_value;
        count_vec[value] += 1;
    }

    let mut total = 0;
    for count in count_vec.iter_mut() {
        let old_count = *count;
        *count = total;
        total += old_count;
    }

    let mut sorted_data = vec![Data::new(); data.len()];
    for d in data.iter() {
        let value = d.value as usize - min_value;
        let index = count_vec[value];
        sorted_data[index] = d.clone();
        count_vec[value] += 1;
    }

    *data = sorted_data;
}

fn merge_sort_par(data: &mut [Data]) {
    if data.len() <= 1 {
        return;
    }

    let mid = data.len() / 2;
    let (left, right) = data.split_at_mut(mid);

    rayon::join(|| merge_sort_par(left), || merge_sort_par(right));
    let mut left = left.to_vec();
    let mut right = right.to_vec();
    merge(&mut left, &mut right, data);
}

fn merge(left: &mut [Data], right: &mut [Data], data: &mut [Data]) {
    let mut left_index = 0;
    let mut right_index = 0;

    for i in 0..data.len() {
        if left_index < left.len() && right_index < right.len() {
            if left[left_index].value < right[right_index].value {
                data[i] = left[left_index].clone();
                left_index += 1;
            } else {
                data[i] = right[right_index].clone();
                right_index += 1;
            }
        } else if left_index < left.len() {
            data[i] = left[left_index].clone();
            left_index += 1;
        } else {
            data[i] = right[right_index].clone();
            right_index += 1;
        }
    }
}

fn read_data(filename: &str) -> Vec<Data> {
    let file = File::open(filename).expect("Unable to open file");
    let mut rdr = ReaderBuilder::new()
        .buffer_capacity(1 << 16) // Set buffer capacity to 64 KB
        .has_headers(true) // Set this to false if your CSV doesn't have headers
        .delimiter(b',') // Change this if your CSV uses a different delimiter
        .quote(b'"') // Change this if your CSV uses a different quoting character
        .escape(Some(b'\\')) // Change this if your CSV uses a different escape character
        .double_quote(true) // Set this to false if your CSV doesn't use double quote escaping
        .flexible(false) // Set this to true if your CSV has a variable number of fields per record
        .from_reader(file);
    let mut record = ByteRecord::new();
    let mut data = Vec::with_capacity(111_438); // Preallocate memory based on an estimate

    while rdr.read_byte_record(&mut record).unwrap() {
        let direction = String::from_utf8_lossy(&record[0]).into_owned();
        let year = String::from_utf8_lossy(&record[1]).parse::<u16>().unwrap();
        let date = String::from_utf8_lossy(&record[2]).into_owned();
        let weekday = String::from_utf8_lossy(&record[3]).into_owned();
        let country = String::from_utf8_lossy(&record[4]).into_owned();
        let comodity = String::from_utf8_lossy(&record[5]).into_owned();
        let transport_mode = String::from_utf8_lossy(&record[6]).into_owned();
        let measure = String::from_utf8_lossy(&record[7]).into_owned();
        let value = String::from_utf8_lossy(&record[8]).parse::<u64>().unwrap();
        let cumulative = String::from_utf8_lossy(&record[9]).parse::<u64>().unwrap();

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

    data
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

fn main() {
    let start = SystemTime::now();
    let mut data_vector = read_data("effects.csv");
    let end = SystemTime::now();
    println!("Reading data took {} ms", end.duration_since(start).unwrap().as_millis());
    
    println!("Select sorting algorithm:");
    println!("1. Counting sort");
    println!("2. Merge sort");
    print!("Enter your choice: ");
    std::io::stdout().flush().unwrap();

    let choice = user_input();

    match choice.as_str() {
        "1" => {
            let start = SystemTime::now();
            counting_sort(&mut data_vector);
            let end = SystemTime::now();
            print_data(&data_vector);
            println!("Counting sort took {} ms", end.duration_since(start).unwrap().as_millis());
            // println!("Counting sort took {} ns", end.duration_since(start).unwrap().as_nanos());

        },
        "2" => {
            let start = SystemTime::now();
            // let sorted_data = merge_sort(&data_vector);
            merge_sort_par(&mut data_vector);
            let end = SystemTime::now();
            // print_data(&sorted_data);
            print_data(&data_vector);
            println!("Merge sort took {} ms", end.duration_since(start).unwrap().as_millis());
        },
        _ => {
            println!("Invalid choice");
        }
    }
}
