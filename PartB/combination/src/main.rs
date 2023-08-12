#[allow(dead_code)]
mod avl_tree;
mod hash_table;
use crate::avl_tree::*;
use crate::hash_table::*;
use std::io::Write;

const FILE: &str = "effects.csv";

fn user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    input
}
fn main() {
    loop {
        println!("---------------------------");
        println!("1. AVL Tree");
        println!("2. Hash Table");
        println!("0. Exit");
        print!("Enter your choice: ");
        std::io::stdout().flush().unwrap();

        let choice = user_input();

        match choice.as_str() {
            "1" => {
                avl_tree_interface(FILE);
            }
            "2" => {
                hash_table_interface(FILE);
            }
            "0" => {
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
        println!("bye bye");
    }
}