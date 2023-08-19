#[allow(unused)]
use std::fs::File;
use std::io::{self, Write};
use std::time::SystemTime;

use csv::{ByteRecord, ReaderBuilder};

mod tests;

#[allow(unused)]
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
    let start = SystemTime::now();
    let data = read_data("cs.csv");
    let end = SystemTime::now();
    println!("Time elapsed: {:?}", end.duration_since(start).unwrap());

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
