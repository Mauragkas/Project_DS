#[allow(unused)]
use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::time::SystemTime;

const MOD: usize = 11;

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

#[derive(Debug, Clone)]
struct Node {
    data: Data,
    next: Option<Box<Node>>
}

impl Node {
    fn new(data: Data) -> Node {
        Node {
            data,
            next: None
        }
    }
}

#[derive(Debug, Clone)]
struct LinkedList {
    first: Option<Box<Node>>,
    last: Option<*mut Node>,
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList {
            first: None,
            last: None,
        }
    }

    fn push_back(&mut self, data: Data) {
        let mut new_node = Box::new(Node::new(data));
        new_node.next = None;

        match self.last {
            None => {
                // The list is empty
                self.first = Some(new_node);
                self.last = Some(self.first.as_mut().unwrap().as_mut() as *mut Node);
            }
            Some(last) => {
                // The list is not empty
                unsafe {
                    (*last).next = Some(new_node);
                    self.last = Some((*last).next.as_mut().unwrap().as_mut() as *mut Node);
                }
            }
        }
    }
}

fn init() -> Vec<LinkedList> {
    let mut hash_table = Vec::with_capacity(MOD);
    for _ in 0..MOD {
        hash_table.push(LinkedList::new());
    }
    hash_table
}

fn hash(date: &str) -> usize {
    let mut sum = 0;
    for c in date.chars() {
        sum += c as usize;
    }
    sum % MOD
}

fn insert(hash_table: &mut Vec<LinkedList>, data: Data) {
    let index = hash(&data.date);
    hash_table[index].push_back(data);
}

fn search(hash_table: &Vec<LinkedList>, date: &str) -> Option<Box<Node>> {
    let index = hash(date);
    let mut current = hash_table[index].first.clone();
    while let Some(node) = current {
        if node.data.date == date {
            return Some(node);
        }
        current = node.next;
    }
    None
}

fn edit(hash_table: &mut Vec<LinkedList>, date: &str, data: Data) {
    let index = hash(date);
    let mut current = hash_table[index].first.as_mut();

    while let Some(node) = current {
        if node.data.date == date {
            node.data = data;
            return;
        }
        current = node.next.as_mut();
    }
}

fn delete(hash_table: &mut Vec<LinkedList>, date: &str) {
    let index = hash(date);
    let mut current = hash_table[index].first.as_mut();

    // If the linked list at the hash index is empty, return.
    if current.is_none() {
        return;
    }

    // If the date to be deleted is at the head of the linked list.
    if current.as_ref().unwrap().data.date == date {
        hash_table[index].first = current.take().unwrap().next.take();
        return;
    }

    // Check the rest of the linked list.
    while let Some(node) = current {
        if let Some(next_node) = node.next.as_mut() {
            if next_node.data.date == date {
                // Node to be deleted is next_node.
                node.next = next_node.next.take();
                return;
            }
        }
        current = node.next.as_mut();
    }
}

fn read_data(filename: &str) -> Vec<LinkedList> {
    let mut reader = match csv::Reader::from_path(filename) {
        Ok(reader) => reader,
        Err(_) => {
            println!("Error reading file");
            exit(1);
        }
    };
    let mut vec = init();

    for result in reader.records() {
        let record = match result {
            Ok(record) => record,
            Err(_) => {
                println!("Error reading record");
                exit(1);
            }
        };
        let data = Data {
            direction: record[0].to_string(),
            year: record[1].parse::<u16>().unwrap(),
            date: record[2].to_string(),
            weekday: record[3].to_string(),
            country: record[4].to_string(),
            comodity: record[5].to_string(),
            transport_mode: record[6].to_string(),
            measure: record[7].to_string(),
            value: record[8].parse::<u64>().unwrap(),
            cumulative: record[9].parse::<u64>().unwrap(),
        };

        // println!("{:?}", data.date);

        insert(&mut vec, data);
    }

    return vec;
}

fn user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    input
}

fn print_data(data: &Data) {
    println!(
        "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
        data.direction,
        data.year,
        data.date,
        data.weekday,
        data.country,
        data.comodity,
        data.transport_mode,
        data.measure,
        data.value,
        data.cumulative
    );
}

// fn to print the vector of linked list
#[allow(dead_code)]
fn print_vec(vec: &Vec<LinkedList>) {
    for (i, list) in vec.iter().enumerate() {
        println!("{}: ", i);
        let mut current = list.first.clone();
        while let Some(node) = current {
            print_data(&node.data);
            current = node.next;
        }
    }
}

fn main() {
    let start = SystemTime::now();
    let mut vec = read_data("effects.csv");
    println!("Time elapsed: {:?}", start.elapsed().unwrap());

    // print_vec(&vec);

    loop {
        println!("---------------------------");
        println!("1. Search");
        println!("2. Edit");
        println!("3. Delete");
        println!("0. Exit");
        print!("Enter your choice: ");
        std::io::stdout().flush().unwrap();

        let choice = user_input();

        match choice.as_str() {
            "1" => {
                print!("Enter date: ");
                std::io::stdout().flush().unwrap();
                let date = user_input();
                let node = match search(&vec, &date) {
                    Some(node) => node,
                    None => {
                        println!("No data found");
                        continue;
                    }
                };
                print_data(&node.data);
            }
            "2" => {
                print!("Enter date: ");
                std::io::stdout().flush().unwrap();
                let date = user_input();
                let node = match search(&vec, &date) {
                    Some(node) => node,
                    None => {
                        println!("No data found");
                        continue;
                    }
                };
                println!("Enter new value: ");
                std::io::stdout().flush().unwrap();
                let value = user_input();
                let mut data = node.data;
                data.value = value.parse::<u64>().unwrap();
                edit(&mut vec, &date, data);
            }
            "3" => {
                print!("Enter date: ");
                std::io::stdout().flush().unwrap();
                let date = user_input();
                delete(&mut vec, &date);
                // print_vec(&vec);
            }
            "0" => {
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }

        // print_vec(&vec);
    }
}
