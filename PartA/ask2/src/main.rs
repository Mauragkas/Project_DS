#![allow(unused)]
use std::fs::File;
use std::io::Write;
use std::time::SystemTime;

use csv::{ReaderBuilder, ByteRecord};

mod tests;

use rayon::prelude::*;

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
fn heapify(data: &mut [Data], n: usize, i: usize) {
    let mut largest = i;
    let l = 2 * i + 1;
    let r = 2 * i + 2;

    if l < n && data[l].cumulative > data[largest].cumulative {
        largest = l;
    }

    if r < n && data[r].cumulative > data[largest].cumulative {
        largest = r;
    }

    if largest != i {
        data.swap(i, largest);
        heapify(data, n, largest);
    }
}

fn heap_sort(data: &mut [Data]) -> &mut [Data] {
    let n = data.len();
    for i in (0..n / 2).rev() {
        heapify(data, n, i);
    }

    for i in (0..n).rev() {
        data.swap(0, i);
        heapify(&mut data[..i], i, 0);
    }

    data
}

fn quick_sort_par(data: &mut [Data]) {
    if data.len() <= 1 {
        return;
    }

    let pivot_index = partition(data);

    let (left, right) = data.split_at_mut(pivot_index);

    rayon::join(|| quick_sort_par(left), || quick_sort_par(&mut right[1..]));
}

fn partition(data: &mut [Data]) -> usize {
    let pivot_index = data.len() / 2;
    data.swap(pivot_index, data.len() - 1);

    let mut i = 0;
    for j in 0..data.len() - 1 {
        if data[j].cumulative <= data[data.len() - 1].cumulative {
            data.swap(i, j);
            i += 1;
        }
    }

    data.swap(i, data.len() - 1);
    i
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
    // let mut input = String::new();

    let start = SystemTime::now();
    let mut data = read_data("effects.csv");
    let end = SystemTime::now();
    println!("Reading data took {} ms", end.duration_since(start).unwrap().as_millis());
    let len = data.len();
    
    println!("--------------------------------");
    println!("{} records", data.len());
    println!("--------------------------------");

    println!("Sort with:");
    println!("1. Heap Sort");
    println!("2. Quick Sort");

    print!("Enter your choice: ");
    std::io::stdout().flush().unwrap();
    let choice = user_input();

    match choice.as_str() {
        "1" => {
            let start = SystemTime::now();
            heap_sort(&mut data);
            let end = SystemTime::now();
            print_data(&data);
            println!("Heap Sort took {} ms", end.duration_since(start).unwrap().as_millis());
        },

        "2" => {
            let start = SystemTime::now();
            quick_sort_par(&mut data);
            let end = SystemTime::now();
            print_data(&data);
            println!("Quick Sort took {} ms", end.duration_since(start).unwrap().as_millis());
        },

        _ => println!("Invalid choice"),
    }

}