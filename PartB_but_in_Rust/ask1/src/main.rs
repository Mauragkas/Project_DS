#[allow(unused)]
use std::fs::File;
use std::io::Write;
use std::time::SystemTime;

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

fn date_to_days(date_str: &str) -> u32 {
    let mut parts = date_str.split('/');
    let day = parts.next().unwrap().parse::<u32>().unwrap();
    let month = parts.next().unwrap().parse::<u32>().unwrap();
    let year = parts.next().unwrap().parse::<u32>().unwrap();
    
    year * 365 + month * 30 + day
}

fn inorder(root: &Option<Box<Node>>) {
    if root.is_none() {
        return;
    }
    let node = root.as_ref().unwrap();
    inorder(&node.left);
    print_data(&node.data);
    inorder(&node.right);
}

// fn to insert a node into a tree
fn insert(root: &mut Option<Box<Node>>, data: &Data) -> () {
    if let Some(ref mut node) = root {
        if date_to_days(&data.date) < date_to_days(&node.data.date) {
            insert(&mut node.left, data);
        } else {
            insert(&mut node.right, data);
        }
    } else {
        *root = Some(Box::new(Node::new(data.clone())));
    }
}

fn min_value_node(node: &Option<Box<Node>>) -> Option<Box<Node>> {
    let mut node = node.as_ref().unwrap().clone();
    while node.left.is_some() {
        node = node.left.unwrap();
    }
    Some(node)
}

fn delete_node(root: Option<Box<Node>>, date_str: &str) -> Option<Box<Node>> {
    let mut root = root;

    if let Some(node) = &mut root {
        let date = date_to_days(date_str);
        let root_date = date_to_days(&node.data.date);

        if date < root_date {
            node.left = delete_node(node.left.take(), date_str);
        } else if date > root_date {
            node.right = delete_node(node.right.take(), date_str);
        } else {
            if node.left.is_none() {
                return node.right.take();
            } else if node.right.is_none() {
                return node.left.take();
            }

            let temp = min_value_node(&node.right);
            let node_data = temp.as_ref().unwrap().data.clone();
            node.data = node_data.clone();
            node.right = delete_node(node.right.take(), &node_data.date);
        }
    }

    root
}

fn search_node(root: &Option<Box<Node>>, date_str: &str) -> Option<Box<Node>> {
    if let Some(node) = root {
        let date = date_to_days(&date_str);
        let root_date = date_to_days(&node.data.date);

        if date == root_date {
            return Some(node.clone());
        } else if date < root_date {
            return search_node(&node.left, date_str);
        } else {
            return search_node(&node.right, date_str);
        }
    }
    None
}

// create a function to edit a node in the tree by editing the data in the address
fn edit_node(root: &mut Option<Box<Node>>, date_str: &str, data: &Data) {
    if let Some(node) = root {
        let date = date_to_days(&date_str);
        let root_date = date_to_days(&node.data.date);

        if date == root_date {
            node.data = data.clone();
        } else if date < root_date {
            edit_node(&mut node.left, date_str, data);
        } else {
            edit_node(&mut node.right, date_str, data);
        }
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

fn user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    input
}

fn main() {
    // get the current time
    let start = SystemTime::now();
    let mut root: Option<Box<Node>> = read_data("effects.csv");
    let stop = SystemTime::now();

    println!("Time taken to read data: {}ns", stop.duration_since(start).unwrap().as_nanos());

    loop {
        println!("---------------------------");
        println!("1. Inorder traversal");
        println!("2. Search");
        println!("3. Edit");
        println!("4. Delete");
        println!("0. Exit");
        print!("Enter your choice: ");
        std::io::stdout().flush().unwrap();

        let choice = user_input();

        match choice.as_str() {
            "1" => inorder(&root),
            "2" => {
                print!("Enter date: ");
                std::io::stdout().flush().unwrap();
                let date = user_input();
                if let Some(node) = search_node(&root, &date) {
                    print_data(&node.data);
                } else {
                    println!("No data found");
                }
            }
            "3" => {
                print!("Enter date: ");
                std::io::stdout().flush().unwrap();
                let date = user_input();
                if let Some(node) = search_node(&root, &date) {
                    let mut data = node.data.clone();
                    print!("Enter new value: ");
                    std::io::stdout().flush().unwrap();
                    data.value = user_input().parse::<u64>().unwrap();
                    edit_node(&mut root, &date, &data);
                } else {
                    println!("No data found");
                }
            }
            "4" => {
                print!("Enter date: ");
                std::io::stdout().flush().unwrap();
                let date = user_input();
                root = delete_node(root, &date);
            }
            "0" => break,
            _ => println!("Invalid choice"),
        }
    }
}