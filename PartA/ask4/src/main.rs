#![allow(unused)]
use std::fs::File;
use std::io::{self, Write};
use std::time::SystemTime;

mod tests;

#[derive(Debug)]
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

fn date_to_days(date_str: &str) -> u32 {
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

fn user_input() -> String {
    let mut input = String::new();
    print!("Enter date (dd/mm/yyyy): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    return input.trim().to_string();
}

fn in_range(data: &Vec<Data>, date: &str) -> bool {
    let max_date = data.iter().map(|x| date_to_days(&x.date)).max().unwrap();
    let min_date = data.iter().map(|x| date_to_days(&x.date)).min().unwrap();

    if date_to_days(date) > max_date || date_to_days(date) < min_date {
        return false;
    }

    return true;
}

fn bis(data: &Vec<Data>, date: &str) -> (bool, usize) {
    let mut left = 0;
    let mut right = data.len() - 1;
    let size = data.len();
    let target = date_to_days(&date);

    let mut next = (size as f32 * (target - date_to_days(&data[left].date)) as f32 / (date_to_days(&data[right].date) - date_to_days(&data[left].date)) as f32).ceil() as usize;

    if next >= size {next = size - 1}

    while target != date_to_days(&data[next].date) {
        let mut i = 0;
        if size <= 3 {
            // linear search
            for i in left..right {
                if date_to_days(&data[i].date) == target {
                    return (true, i);
                }
            }
            return (false, 0);
        }
        if target >= date_to_days(&data[next].date) {
            while target > date_to_days(&data[next + i * ((size as f32).sqrt()) as usize -1].date) {
                i += 1;
                right = next + i * ((size as f32).sqrt()) as usize;
                if right >= size {right = size - 1}
                left = next + (i - 1) * ((size as f32).sqrt()) as usize;
            }           
        } else if target < date_to_days(&data[next].date) {
            while target < date_to_days(&data[next - i * ((size as f32).sqrt().floor() as usize) + 1].date) {
                i += 1;

                right = next - (i - 1) * ((size as f32).sqrt()) as usize;
                if right >= size {right = size - 1}

                left = next - i * ((size as f32).sqrt()) as usize;

                next = left + (((target - date_to_days(&data[left].date)) as f32 / (date_to_days(&data[right].date) - date_to_days(&data[left].date)) as f32) * (right - left + 1) as f32).ceil() as usize - 1;
                if next < 1 {next = 1}
            }           
        } 
    }

    if target == date_to_days(&data[next].date) {
        return (true, next);
    }

    return (false, 0);
}
fn main() {
    let data = read_data("cs.csv");

    let input = user_input();

    if !in_range(&data, &input) {
        println!("Date out of range");
        return;
    }

    let start = SystemTime::now();
    let (found, index) = bis(&data, &input);

    if found {
        println!("{index}");
        println!("{:?}", data[index]);
        let end = SystemTime::now();
        println!("{}ns", end.duration_since(start).unwrap().as_nanos());
    } else {
        println!("Date not found");
    }

}
