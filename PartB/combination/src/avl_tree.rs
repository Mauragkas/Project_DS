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

fn date_to_days(date_str: &str) -> Option<u32> {
    let mut parts = date_str.split('/');
    let day = parts.next()?.parse::<u32>().ok()?;
    let month = parts.next()?.parse::<u32>().ok()?;
    let year = parts.next()?.parse::<u32>().ok()?;
    
    Some(year * 365 + month * 30 + day)
}

fn height(node: &Option<Box<Node>>) -> i32 {
    match node {
        Some(n) => n.height,
        None => 0,
    }
}

fn balance_factor(node: &Node) -> i32 {
    height(&node.left) - height(&node.right)
}

fn update_height(node: &mut Box<Node>) {
    let hl = height(&node.left);
    let hr = height(&node.right);
    node.height = std::cmp::max(hl, hr) + 1;
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
            height: 1,
        }));
        return;
    }
    if date_to_days(&data.date) < date_to_days(&root.as_ref().unwrap().data.date) {
        insert(&mut root.as_mut().unwrap().left, data);
    } else {
        insert(&mut root.as_mut().unwrap().right, data);
    }
    update_height(root.as_mut().unwrap());
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
            let min_data = min_node.data.clone();
            delete_node(&mut root.as_mut().unwrap().right, &min_data.date);
            root.as_mut().unwrap().data = min_data;
        }
    }
    if root.is_some() {
        *root = Some(balance(root.take().unwrap()));
    }
}

fn edit_node(node: &mut Option<Box<Node>>, date_str: &str) {
    if let Some(ref mut node_box) = node {
        if node_box.data.date == date_str {
            // The date_str node is found, now we can update its data
            print!("Enter the new Value: ");
            std::io::stdout().flush().unwrap();
            let value = user_input();
            let new_value = match value.parse::<u64>() {
                Ok(v) => v,
                Err(_) => {
                    println!("Invalid value.");
                    return;
                }
            };
            node_box.data.value = new_value;
        } else if date_to_days(date_str) < date_to_days(&node_box.data.date) {
            edit_node(&mut node_box.left, date_str);
        } else {
            edit_node(&mut node_box.right, date_str);
        }
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

fn find_min_value_node(root: &Option<Box<Node>>) -> Option<&Box<Node>> {
    match root {
        Some(node) => {
            let left_min = find_min_value_node(&node.left);
            let right_min = find_min_value_node(&node.right);

            let mut min_node = node;
            if let Some(l_node) = left_min {
                if l_node.data.value < min_node.data.value {
                    min_node = l_node;
                }
            }

            if let Some(r_node) = right_min {
                if r_node.data.value < min_node.data.value {
                    min_node = r_node;
                }
            }

            Some(min_node)
        },
        None => None,
    }
}

fn find_max_value_node(root: &Option<Box<Node>>) -> Option<&Box<Node>> {
    match root {
        Some(node) => {
            let left_max = find_max_value_node(&node.left);
            let right_max = find_max_value_node(&node.right);

            let mut max_node = node;
            if let Some(l_node) = left_max {
                if l_node.data.value > max_node.data.value {
                    max_node = l_node;
                }
            }

            if let Some(r_node) = right_max {
                if r_node.data.value > max_node.data.value {
                    max_node = r_node;
                }
            }

            Some(max_node)
        },
        None => None,
    }
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

pub fn avl_tree_interface(filename: &str) {
    let start = SystemTime::now();
    let mut root = read_data(filename);
    let stop = SystemTime::now();

    println!("Time taken to read data: {}ms", stop.duration_since(start).unwrap().as_millis());

    loop {
        println!("---------------------------");
        println!("1. Inorder traversal");
        println!("2. Search");
        println!("3. Edit");
        println!("4. Delete");
        println!("5. Find the data with the MAX value");
        println!("6. Find the data with the MIN value");
        println!("0. Back");
        print!("Enter your choice: ");
        std::io::stdout().flush().unwrap();

        let choice = user_input();

        match choice.as_str() {
            "1" => root.as_ref().unwrap().inorder(),
            "2" => {
                print!("Enter date: ");
                std::io::stdout().flush().unwrap();
                let date = user_input();

                if date_to_days(&date).is_none() {
                    println!("Invalid date format");
                    continue;
                }

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
                
                if date_to_days(&date).is_none() {
                    println!("Invalid date format");
                    continue;
                }

                root.as_mut().unwrap().edit(&date);
                println!("Data updated");
            }            
            "4" => {
                print!("Enter date: ");
                std::io::stdout().flush().unwrap();
                let date = user_input();

                if date_to_days(&date).is_none() {
                    println!("Invalid date format");
                    continue;
                }

                root.as_mut().unwrap().delete(&date);
                println!("Data deleted");
            }
            "5" => {
                let node = find_max_value_node(&root.as_ref().unwrap().root);
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
            "6" => {
                let node = find_min_value_node(&root.as_ref().unwrap().root);
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
            "0" => break,
            _ => println!("Invalid choice"),
        }
        println!("");
    }
}

