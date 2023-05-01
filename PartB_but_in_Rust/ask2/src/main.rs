#[allow(unused)]
use std::fs::File;
use std::io::Write;

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
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(data: Data) -> Node {
        Node {
            data,
            left: None,
            right: None,
        }
    }
}

fn insert(root: &mut Option<Box<Node>>, data: &Data) -> () {
    if let Some(ref mut node) = root {
        if &data.value < &node.data.value {
            insert(&mut node.left, data);
        } else {
            insert(&mut node.right, data);
        }
    } else {
        *root = Some(Box::new(Node::new(data.clone())));
    }
}

fn node_with_min_value(root: &Option<Box<Node>>) -> Option<Box<Node>> {
    let mut current = root.as_ref().unwrap();
    while let Some(node) = &current.left {
        current = node;
    }
    return Some(current.clone());
}

fn node_with_max_value(root: &Option<Box<Node>>) -> Option<Box<Node>> {
    let mut current = root.as_ref().unwrap();
    while let Some(node) = &current.right {
        current = node;
    }
    return Some(current.clone());
}

// fn to store nodes in a vector with the same value
fn nodes_with_same_value<'a>(root: &'a Option<Box<Node>>, value: &u64, nodes: &mut Vec<&'a Node>) {
    if let Some(ref node) = *root {
        if &node.data.value == value {
            nodes.push(node.as_ref());
        }
        nodes_with_same_value(&node.left, value, nodes);
        nodes_with_same_value(&node.right, value, nodes);
    }
}

fn read_data(filename: &str) -> Option<Box<Node>> {
    let mut reader = match csv::Reader::from_path(filename) {
        Ok(reader) => reader,
        Err(_) => {
            panic!("Error reading file");
        }
    };
    let mut root = None;

    for result in reader.records() {
        let record = match result {
            Ok(record) => record,
            Err(_) => {
                panic!("Error reading record");
            }
        };
        let data = Data {
            direction: record.get(0).unwrap().to_string(),
            year: record.get(1).unwrap().parse::<u16>().unwrap(),
            date: record.get(2).unwrap().to_string(),
            weekday: record.get(3).unwrap().to_string(),
            country: record.get(4).unwrap().to_string(),
            comodity: record.get(5).unwrap().to_string(),
            transport_mode: record.get(6).unwrap().to_string(),
            measure: record.get(7).unwrap().to_string(),
            value: record.get(8).unwrap().parse::<u64>().unwrap(),
            cumulative: record.get(9).unwrap().parse::<u64>().unwrap(),
        };

        insert(&mut root, &data);
    }

    return root;
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

fn main() {
    let root = read_data("effects.csv");
    
    loop {
        println!("---------------------------");
        println!("1. find the data with the MAX value");
        println!("2. find the data with the MIN value");
        println!("0. exit");
        println!("---------------------------");
        print!("Enter your choice: ");
        std::io::stdout().flush().unwrap();
        let choice = user_input();

        match choice.as_str() {
            "1" => {
                let node = node_with_max_value(&root);
                match node {
                    Some(node) => {
                        // print_data(&node.data);
                        let mut nodes = Vec::new();
                        nodes_with_same_value(&root, &node.data.value, &mut nodes);
                        let mut i = 0;
                        for node in nodes {
                            if i < 10 {
                                print_data(&node.data);
                                i += 1;
                            }
                        }
                    }
                    None => {
                        println!("No data found");
                    }
                }
            }
            "2" => {
                let node = node_with_min_value(&root);
                match node {
                    Some(node) => {
                        // print_data(&node.data);
                        let mut nodes = Vec::new();
                        nodes_with_same_value(&root, &node.data.value, &mut nodes);
                        for node in nodes {
                            print_data(&node.data);
                        }
                    }
                    None => {
                        println!("No data found");
                    }
                }
            }
            "0" => {
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }

    println!("Bye!");
}