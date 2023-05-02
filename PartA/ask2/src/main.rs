#[allow(unused)]
use std::fs::File;
use std::io::Write;
use std::time::SystemTime;

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

fn heapify(data: &mut Vec<Data>, n: usize, i: usize) {
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

fn heap_sort(data: &mut Vec<Data>) -> &Vec<Data> {
    let n = data.len();
    for i in (0..n / 2).rev() {
        heapify(data, n, i);
    }

    for i in (0..n).rev() {
        data.swap(0, i);
        heapify(data, i, 0);
    }

    return data;
}

fn partition(data: &mut Vec<Data>, low: usize, high: usize) -> usize {
    let pivot = data[high].cumulative;
    let mut i = low;
    for j in low..high {
        if data[j].cumulative < pivot {
            data.swap(i, j);
            i += 1;
        }
    }
    data.swap(i, high);
    return i;
}

fn quick_sort(data: &mut Vec<Data>, low: usize, high: usize) -> &Vec<Data> {
    if low < high {
        let pi = partition(data, low, high);
        // println!("{} {} {}", low, high, pi);
        if pi > low {
            quick_sort(data, low, pi - 1);
        }
        quick_sort(data, pi + 1, high);
    }
    return data;
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
    // let mut input = String::new();

    let mut data = read_data("effects.csv");
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
            quick_sort(&mut data, 0, len - 1);
            let end = SystemTime::now();
            print_data(&data);
            println!("Quick Sort took {} ms", end.duration_since(start).unwrap().as_millis());
        },

        _ => println!("Invalid choice"),
    }

}
