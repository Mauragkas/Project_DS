#[allow(unused)]
use std::fs::File;
use std::io::Write;

const MOD: usize = 11;

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

fn init() -> Vec<Option<Box<Node>>> {
    return vec![None; MOD];
}

fn hashing(str: &str) -> usize {
    let mut sum = 0;
    for c in str.chars() {
        sum += c as usize;
    }
    return sum % MOD;
}

fn insert(v: &mut Vec<Option<Box<Node>>>, data: Data) -> () {
    let index = hashing(&data.date);
    let node = Node::new(data);
    match v[index] {
        None => {
            v[index] = Some(Box::new(node));
        },
        Some(ref mut head) => {
            let mut current = head;
            while let Some(ref mut next) = current.next {
                current = next;
            }
            current.next = Some(Box::new(node));
        }
    }
}

fn search(v: &Vec<Option<Box<Node>>>, date: &str) -> Option<Box<Node>> {
    let index = hashing(date);
    match &v[index] {
        None => {
            // println!("No data for this date");
            return None;
        },
        Some(head) => {
            let mut current = head;
            while let Some(next) = &current.next {
                if &next.data.date == date {
                    return Some(next.clone());
                }
                current = next;
            }
            return None;
        }
    }
}

fn edit(v: &mut Vec<Option<Box<Node>>>, date: &str, data: Data) -> () {
    let index = hashing(date);
    match &mut v[index] {
        None => {
            println!("No data for this date");
        },
        Some(head) => {
            let mut current = head;
            while let Some(next) = &mut current.next {
                if &next.data.date == date {
                    next.data = data;
                    return;
                }
                current = next;
            }
        }
    }
}

fn delete_and_attach_next(head: Option<Box<Node>>) -> Option<Box<Node>> {
    if let Some(mut curr) = head {
        if let Some(mut next) = curr.next.take() {
            curr.next = next.next.take();
            next.next = Some(curr);
            Some(next)
        } else {
            Some(curr)
        }
    } else {
        None
    }
}

fn delete(v: &mut Vec<Option<Box<Node>>>, date: &str) -> Vec<Option<Box<Node>>> {
    let index = hashing(date);
    
    // if the key is not in the vector, return the vector as it is
    if v[index].is_none() {
        return v.clone();
    } else {
        // if the key is the first node in the linked list
        if v[index].as_ref().unwrap().data.date == date {
            let mut head = v[index].take();
            let mut next = head.as_mut().unwrap().next.take();
            head = delete_and_attach_next(head);
            v[index] = head;
            while let Some(mut curr) = next {
                next = curr.next.take();
                curr = delete_and_attach_next(Some(curr)).unwrap();
                insert(v, curr.data);
            }
        } else {
            let mut head = v[index].take();
            let mut next = head.as_mut().unwrap().next.take();
            while let Some(mut curr) = next {
                if curr.data.date == date {
                    next = curr.next.take();
                    curr = delete_and_attach_next(Some(curr)).unwrap();
                    insert(v, curr.data);
                } else {
                    next = curr.next.take();
                    curr = delete_and_attach_next(Some(curr)).unwrap();
                    insert(v, curr.data);
                }
            }
        }

        return v.clone();
    }
}

fn read_data(filename: &str) -> Vec<Option<Box<Node>>> {
    let mut reader = match csv::Reader::from_path(filename) {
        Ok(reader) => reader,
        Err(_) => {
            panic!("Error reading file");
        }
    };
    let mut vec = init();

    for result in reader.records() {
        let record = match result {
            Ok(record) => record,
            Err(_) => {
                panic!("Error reading record");
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

fn main() {
    let mut vec = read_data("effects.csv");
    
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
                        println!("No data for this date");
                        continue;
                    }
                };
                println!("{:?}", node.data);
            },
            "2" => {
                print!("Enter date: ");
                std::io::stdout().flush().unwrap();
                let date = user_input();
                let node = match search(&vec, &date) {
                    Some(node) => node,
                    None => {
                        println!("No data for this date");
                        continue;
                    }
                };
                println!("Enter new value: ");
                std::io::stdout().flush().unwrap();
                let value = user_input();
                let mut data = node.data;
                data.value = value.parse::<u64>().unwrap();
                edit(&mut vec, &date, data);
            },
            "3" => {
                print!("Enter date: ");
                std::io::stdout().flush().unwrap();
                let date = user_input();
                vec = delete(&mut vec, &date);
            },
            "0" => {
                break;
            },
            _ => {
                println!("Invalid choice");
            }
        }
    }
}
