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

    fn inorder(&self) {
        inorder(&self.root);
    }

    fn search(&self, date_str: &str) -> Option<&Node> {
        search_node(&self.root, date_str)
    }

    fn delete(&mut self, date_str: &str) {
        delete_node(&mut self.root, date_str);
    }

    fn edit(&mut self, date_str: &str) {
        edit_node(&mut self.root, date_str);
    }

}

fn date_to_days(date_str: &str) -> u32 {
    let mut parts = date_str.split('/');
    let day = parts.next().unwrap().parse::<u32>().unwrap();
    let month = parts.next().unwrap().parse::<u32>().unwrap();
    let year = parts.next().unwrap().parse::<u32>().unwrap();
    
    year * 365 + month * 30 + day
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
    if date_to_days(&data.date) < date_to_days(&root.as_ref().unwrap().data.date) {
        insert(&mut root.as_mut().unwrap().left, data);
    } else {
        insert(&mut root.as_mut().unwrap().right, data);
    }
    *root = Some(balance(root.take().unwrap()));
}

fn inorder(root: &Option<Box<Node>>) {
    if root.is_none() {
        return;
    }
    inorder(&root.as_ref().unwrap().left);
    print_data(&root.as_ref().unwrap().data);
    inorder(&root.as_ref().unwrap().right);
}

fn search_node<'a>(root: &'a Option<Box<Node>>, date_str: &str) -> Option<&'a Node> {
    if root.is_none() {
        return None;
    }
    if date_to_days(date_str) < date_to_days(&root.as_ref().unwrap().data.date) {
        return search_node(&root.as_ref().unwrap().left, date_str);
    } else if date_to_days(date_str) > date_to_days(&root.as_ref().unwrap().data.date) {
        return search_node(&root.as_ref().unwrap().right, date_str);
    } else {
        return root.as_deref();
    }
}

fn delete_node(root: &mut Option<Box<Node>>, date_str: &str) {
    if root.is_none() {
        return;
    }
    if date_to_days(date_str) < date_to_days(&root.as_ref().unwrap().data.date) {
        delete_node(&mut root.as_mut().unwrap().left, date_str);
    } else if date_to_days(date_str) > date_to_days(&root.as_ref().unwrap().data.date) {
        delete_node(&mut root.as_mut().unwrap().right, date_str);
    } else {
        if root.as_ref().unwrap().left.is_none() {
            *root = root.as_mut().unwrap().right.take();
        } else if root.as_ref().unwrap().right.is_none() {
            *root = root.as_mut().unwrap().left.take();
        } else {
            let mut min_node = root.as_mut().unwrap().right.as_mut().unwrap();
            while min_node.left.is_some() {
                min_node = min_node.left.as_mut().unwrap();
            }
            let min_data = min_node.clone();
            delete_node(&mut root.as_mut().unwrap().right, &min_data.data.date);
            root.as_mut().unwrap().data = min_data.data;
        }
    }
    if root.is_some() {
        *root = Some(balance(root.take().unwrap()));
    }
}

fn edit_node(root: &mut Option<Box<Node>>, date_str: &str) {
    if root.is_none() {
        return;
    }
    if let Some(node) = search_node(root, date_str) {
        // The date_str node is found, now we can update its data
        let mut date_str_node = node.clone();

        print!("Enter the new Value: ");
        std::io::stdout().flush().unwrap();
        let value = user_input();
        date_str_node.data.value = value.parse::<u64>().unwrap();

        // Now we need to delete the original node and insert the updated node
        delete_node(root, date_str);
        insert(root, &date_str_node.data);
    } else {
        println!("Date not found");
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

        // println!("{}", data.date);
        tree.insert(&data);
    }

    return Some(tree);
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
    let start = SystemTime::now();
    let mut root = read_data("effects.csv");
    let stop = SystemTime::now();

    println!("Time taken to read data: {}ms", stop.duration_since(start).unwrap().as_millis());

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
            "1" => root.as_ref().unwrap().inorder(),
            "2" => {
                print!("Enter date: ");
                std::io::stdout().flush().unwrap();
                let date = user_input();

                if let Some(node) = root.as_ref().unwrap().search(&date) {
                    let node_data = &node.data;
                    print_data(&node_data);
                } else {
                    println!("No data found");
                }
            }
            "3" => {
                print!("Enter date: ");
                std::io::stdout().flush().unwrap();
                let date = user_input();
                
                root.as_mut().unwrap().edit(&date);
                println!("Data updated");
            }            
            "4" => {
                print!("Enter date: ");
                std::io::stdout().flush().unwrap();
                let date = user_input();

                root.as_mut().unwrap().delete(&date);
                println!("Data deleted");
            }
            "0" => break,
            _ => println!("Invalid choice"),
        }
        println!("");
    }
}

