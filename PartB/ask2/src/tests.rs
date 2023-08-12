#![allow(unused)]

use std::rc::Rc;

use crate::Data;
use crate::AvlTree;
use crate::height;
use crate::node_with_max_value;
use crate::node_with_min_value;
use crate::nodes_with_same_value;
use crate:: read_data;


#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_tree() -> AvlTree {
        let mut tree = AvlTree::new();

        let data1 = Rc::new(Data {
            direction: "import".to_string(),
            year: 2021,
            date: "01-01-2021".to_string(),
            weekday: "Friday".to_string(),
            country: "US".to_string(),
            comodity: "electronics".to_string(),
            transport_mode: "air".to_string(),
            measure: "ton".to_string(),
            value: 5,
            cumulative: 5,
        });

        let data2 = Rc::new(Data {
            direction: "import".to_string(),
            year: 2021,
            date: "01-01-2021".to_string(),
            weekday: "Friday".to_string(),
            country: "US".to_string(),
            comodity: "electronics".to_string(),
            transport_mode: "air".to_string(),
            measure: "ton".to_string(),
            value: 3,
            cumulative: 5,
        });

        // ... you can create more data samples if needed

        tree.insert(data1);
        tree.insert(data2);
        // ... insert other data

        tree
    }

    #[test]
    fn test_insert_and_balance() {
        let tree = create_sample_tree();
        assert_eq!(height(&tree.root), 1); // This might change based on your data
        // Add more assertions based on the data you've inserted
    }

    #[test]
    fn test_max_value() {
        let tree = create_sample_tree();
        let max_node = node_with_max_value(&tree.root);
        assert_eq!(max_node.unwrap().data.value, 5);
    }

    #[test]
    fn test_min_value() {
        let tree = create_sample_tree();
        let min_node = node_with_min_value(&tree.root);
        assert_eq!(min_node.unwrap().data.value, 3);
    }

    #[test]
    fn test_nodes_with_value() {
        let tree = create_sample_tree();
        let mut nodes = Vec::new();
        nodes_with_same_value(&tree.root, &5, &mut nodes);
        assert_eq!(nodes.len(), 1); // If you've only inserted one node with value 5
        assert_eq!(nodes[0].data.value, 5);
    }

    #[test]
    fn test_read_data_from_csv() {
        // For this, you would ideally have a sample CSV file for testing
        let tree = read_data("test.csv").unwrap();
        // Insert some assertions based on the data in your test CSV
        // Example:
        assert_eq!(height(&tree.root), 4); 
    }
}
