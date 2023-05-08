#[allow(unused)]
use std::fs::File;
use std::io::Write;
use std::process::exit;
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
    height: i32,
}

#[derive(Debug)]
struct AvlTree {
    root: Option<Box<Node>>,
}

impl AvlTree {
    fn new() -> AvlTree {
        AvlTree { root: None }
    }

    fn insert(&mut self, data: &Data) {
        insert(&mut self.root, data);
    }

}

fn height(node: &Option<Box<Node>>) -> i32 {
    match node {
        Some(n) => n.height,
        None => -1,
    }
}

fn balance_factor(node: &Node) -> i32 {
    height(&node.left) - height(&node.right)
}

fn update_height(node: &mut Box<Node>) {
    node.height = 1 + std::cmp::max(height(&node.left), height(&node.right));
}

fn rotate_left(mut node: Box<Node>) -> Box<Node> {
    let mut new_root = node.right.take().unwrap();
    node.right = new_root.left.take();
    update_height(&mut node);
    update_height(&mut new_root);
    new_root.left = Some(node);
    new_root
}

fn rotate_right(mut node: Box<Node>) -> Box<Node> {
    let mut new_root = node.left.take().unwrap();
    node.left = new_root.right.take();
    update_height(&mut node);
    update_height(&mut new_root);
    new_root.right = Some(node);
    new_root
}

fn balance(mut node: Box<Node>) -> Box<Node> {
    update_height(&mut node);
    if balance_factor(&node) > 1 {
        if balance_factor(&node.left.as_ref().unwrap()) < 0 {
            node.left = Some(rotate_left(node.left.unwrap()));
        }
        return rotate_right(node);
    }
    if balance_factor(&node) < -1 {
        if balance_factor(&node.right.as_ref().unwrap()) > 0 {
            node.right = Some(rotate_right(node.right.unwrap()));
        }
        return rotate_left(node);
    }
    node
}

fn insert(root: &mut Option<Box<Node>>, data: &Data) {
    if root.is_none() {
        *root = Some(Box::new(Node {
            data: data.clone(),
            left: None,
            right: None,
            height: 0,
        }));
        return;
    }
    if data.value < root.as_ref().unwrap().data.value {
        insert(&mut root.as_mut().unwrap().left, data);
    } else {
        insert(&mut root.as_mut().unwrap().right, data);
    }
    *root = Some(balance(root.take().unwrap()));
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

fn nodes_with_same_value<'a>(root: &'a Option<Box<Node>>, value: &u64, nodes: &mut Vec<&'a Node>) {
    if let Some(ref node) = *root {
        if &node.data.value == value {
            nodes.push(node.as_ref());
        }
        nodes_with_same_value(&node.left, value, nodes);
        nodes_with_same_value(&node.right, value, nodes);
    }
}

fn read_data(filename: &str) -> Option<AvlTree> {
    let mut reader = match csv::Reader::from_path(filename) {
        Ok(reader) => reader,
        Err(_) => {
            println!("Error reading file");
            exit(1);
        }
    };
    let mut tree = AvlTree::new();

    for result in reader.records() {
        let record = match result {
            Ok(record) => record,
            Err(_) => {
                println!("Error reading record");
                exit(1);
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

        tree.insert(&data);
    }

    return Some(tree);
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
    let start = SystemTime::now();
    let root = read_data("effects.csv");
    println!("Time elapsed: {:?}", start.elapsed().unwrap());
    
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
                let node = node_with_max_value(&root.as_ref().unwrap().root);
                match node {
                    Some(node) => {
                        let mut nodes = Vec::new();
                        nodes_with_same_value(&root.as_ref().unwrap().root, &node.data.value, &mut nodes);
                        nodes.into_iter().take(10).for_each(|node| {
                            print_data(&node.data);
                        });
                    }
                    None => {
                        println!("No data found");
                    }
                }
            }
            "2" => {
                let node = node_with_min_value(&root.as_ref().unwrap().root);
                match node {
                    Some(node) => {
                        let mut nodes = Vec::new();
                        nodes_with_same_value(&root.as_ref().unwrap().root, &node.data.value, &mut nodes);
                        nodes.into_iter().take(10).for_each(|node| {
                            print_data(&node.data);
                        });
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
        println!()
    }

    println!("Bye!");
}
