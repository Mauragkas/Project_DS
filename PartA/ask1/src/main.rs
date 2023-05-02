#[allow(unused)]
use std::fs::File;
use std::io::Write;
use std::time::SystemTime;
use std::sync::{Arc, Mutex};
use std::thread;

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
    let max_value = data.iter().map(|d| d.value).max().unwrap() as usize;

    let min_value = data.iter().map(|d| d.value).min().unwrap() as usize;

    // time for counting sort
    let start = SystemTime::now();

    let mut count_vec = vec![0; max_value - min_value + 1];
    data.iter().for_each(|d| count_vec[d.value as usize - min_value] += 1);

    for i in 1..count_vec.len() {
        count_vec[i] += count_vec[i - 1];
    }

    let end = SystemTime::now();
    let time = end.duration_since(start).unwrap().as_millis();
    println!("Time for counting sort: {} ms", time);

    let mut sorted_data = vec![Data::new(); data.len()];
    data.iter().rev().for_each(|d| {
        sorted_data[count_vec[d.value as usize - min_value] - 1] = d.clone();
        count_vec[d.value as usize - min_value] -= 1;
    });

    data.iter_mut().enumerate().for_each(|(i, d)| *d = sorted_data[i].clone());
}

fn merge(left_vec: &Vec<Data>, right_vec: &Vec<Data>) -> Vec<Data> {
    let mut merged_vec = Vec::new();
    let mut left_index = 0;
    let mut right_index = 0;
    while left_index < left_vec.len() && right_index < right_vec.len() {
        let left_value = left_vec[left_index].value;
        let right_value = right_vec[right_index].value;
        if left_value < right_value {
            merged_vec.push(left_vec[left_index].clone());
            left_index += 1;
        } else {
            merged_vec.push(right_vec[right_index].clone());
            right_index += 1;
        }
    }
    while left_index < left_vec.len() {
        merged_vec.push(left_vec[left_index].clone());
        left_index += 1;
    }
    while right_index < right_vec.len() {
        merged_vec.push(right_vec[right_index].clone());
        right_index += 1;
    }
    return merged_vec;
}

fn merge_sort(data: &Vec<Data>) -> Vec<Data> {
    if data.len() <= 1 {
        return data.clone();
    }
    let mid = data.len() / 2;
    let left_vec = merge_sort(&data[0..mid].to_vec());
    let right_vec = merge_sort(&data[mid..].to_vec());
    return merge(&left_vec, &right_vec);
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
            // let sorted_data = counting_sort(&data_vector);
            counting_sort(&mut data_vector);
            let end = SystemTime::now();
            // print_data(&data_vector);
            println!("Counting sort took {} ms", end.duration_since(start).unwrap().as_millis());

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
