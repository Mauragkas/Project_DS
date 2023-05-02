#[allow(unused)]
use std::fs::File;
use std::io::{self, Write};

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

        data.push(Data {
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

fn binary_search(data: &Vec<Data>, low: usize, high: usize, date: &str) -> usize {
    if high < low {
        return low;
    }

    let mid = (low + high) / 2;
    let mid_days = convert_date_to_days(&data[mid].date);

    if mid_days == convert_date_to_days(date) {
        return mid;
    } else if mid_days > convert_date_to_days(date) {
        return binary_search(data, low, mid - 1, date);
    } else {
        return binary_search(data, mid + 1, high, date);
    }
}

fn interpolation_search(data: &Vec<Data>, low: usize, high: usize, date: &str) -> usize {
    if high < low {
        return low;
    }

    let mid = low + ((convert_date_to_days(date) - convert_date_to_days(&data[low].date)) * (high as u32 - low as u32) / (convert_date_to_days(&data[high].date) - convert_date_to_days(&data[low].date))) as usize;
    let mid_days = convert_date_to_days(&data[mid].date);

    if mid_days == convert_date_to_days(date) {
        return mid;
    } else if mid_days > convert_date_to_days(date) {
        return interpolation_search(data, low, mid - 1, date);
    } else {
        return interpolation_search(data, mid + 1, high, date);
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
    let max_date = data.iter().map(|x| convert_date_to_days(&x.date)).max().unwrap();
    let min_date = data.iter().map(|x| convert_date_to_days(&x.date)).min().unwrap();

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
    let data = read_data("cs.csv");
    
    let input = user_input();

    if !in_range(&data, input.trim()) {
        println!("Date out of range");
        return;
    }

    let index = binary_search(&data, 0, data.len() - 1, input.trim()) as usize;
    println!("\nbinary search Done!");
    
    print_data_line(&data, index);

    let index = interpolation_search(&data, 0, data.len() - 1, input.trim()) as usize;
    println!("\ninterpolation search Done!");

    let index = index;
    print_data_line(&data, index);
}
