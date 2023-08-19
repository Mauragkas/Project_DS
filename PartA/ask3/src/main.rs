#[allow(unused)]
use std::fs::File;
use std::io::{self, Write};
use std::time::SystemTime;

use csv::{ByteRecord, ReaderBuilder};

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

fn convert_date_to_days(date_str: &str) -> u32 {
    let mut parts = date_str.split('/');
    let day = parts.next().unwrap().parse::<u32>().unwrap();
    let month = parts.next().unwrap().parse::<u32>().unwrap();
    let year = parts.next().unwrap().parse::<u32>().unwrap();
    
    year * 365 + month * 30 + day
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

fn binary_search(data: &Vec<Data>, low: usize, high: usize, date_key: u32) -> usize {
    if high < low {
        return low;
    }

    let mid = (low + high) / 2;
    let mid_days = convert_date_to_days(&data[mid].date);

    if mid_days == date_key {
        return mid;
    } else if mid_days > date_key {
        return binary_search(data, low, mid - 1, date_key);
    } else {
        return binary_search(data, mid + 1, high, date_key);
    }
}

fn interpolation_search(data: &Vec<Data>, low: usize, high: usize, date_key: u32) -> usize {
    if high < low {
        return low;
    }

    let mid = low + ((date_key - convert_date_to_days(&data[low].date)) * (high as u32 - low as u32) / (convert_date_to_days(&data[high].date) - convert_date_to_days(&data[low].date))) as usize;
    let mid_days = convert_date_to_days(&data[mid].date);

    if mid_days == date_key {
        return mid;
    } else if mid_days > date_key {
        return interpolation_search(data, low, mid - 1, date_key);
    } else {
        return interpolation_search(data, mid + 1, high, date_key);
    }
}

fn user_input() -> String {
    let mut input = String::new();
    print!("Enter date (dd/mm/yyyy): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    return input;
}

fn in_range(data: &Vec<Data>, date: &str) -> bool {
    let max_date = convert_date_to_days(data[data.len() - 1].date.as_str());
    let min_date = convert_date_to_days(data[0].date.as_str());

    if convert_date_to_days(date) > max_date || convert_date_to_days(date) < min_date {
        return false;
    }

    return true;
}

fn print_data_line(data: &Vec<Data>, index: usize) {
    println!("Index-> {}: {}|{}|{}|{}|{}|{}|{}|{}|{}|{}", 
        index,
        data[index].direction, 
        data[index].year, 
        data[index].date, 
        data[index].weekday, 
        data[index].country, 
        data[index].comodity, 
        data[index].transport_mode, 
        data[index].measure, data[index].value, 
        data[index].cumulative
    );
}

fn main() {
    let start = SystemTime::now();
    let data = read_data("cs.csv");
    let end = SystemTime::now();

    println!("Time elapsed: {:?}", end.duration_since(start).unwrap());
    
    let input = user_input();

    if !in_range(&data, input.trim()) {
        println!("Date out of range");
        return;
    }

    // Convert the input date to days for faster comparison later on
    let date_key = convert_date_to_days(input.trim());

    let start = SystemTime::now();
    let index = binary_search(&data, 0, data.len() - 1, date_key) as usize;
    let end = SystemTime::now();
    println!("\nbinary search Done!");
    println!("Time elapsed: {:?}ns", end.duration_since(start).unwrap().as_nanos());
    
    print_data_line(&data, index);

    let start = SystemTime::now();
    let index = interpolation_search(&data, 0, data.len() - 1, date_key) as usize;
    let end = SystemTime::now();
    println!("\ninterpolation search Done!");
    println!("Time elapsed: {:?}ns", end.duration_since(start).unwrap().as_nanos());

    let index = index;
    print_data_line(&data, index);
}
