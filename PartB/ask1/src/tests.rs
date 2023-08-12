#![allow(unused)]
use crate::Data;
use crate::date_to_days;
use crate::AvlTree;
use crate::read_data;
use crate::height;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_to_days() {
        assert_eq!(date_to_days("01/01/2020"), Some(737331));
        assert_eq!(date_to_days("31/12/2020"), Some(737691));
        assert_eq!(date_to_days("invalid date"), None);
    }

    #[test]
    fn test_avl_insertion() {
        let mut tree = AvlTree::new();
        let data = Data {
            direction: "Import".to_string(),
            year: 2023,
            date: "01/01/2023".to_string(),
            weekday: "Sunday".to_string(),
            country: "USA".to_string(),
            comodity: "Electronics".to_string(),
            transport_mode: "Air".to_string(),
            measure: "Tons".to_string(),
            value: 1000,
            cumulative: 1000,
        };

        tree.insert(&data);
        assert_eq!(tree.root.as_ref().unwrap().data, data);
        assert_eq!(tree.root.as_ref().unwrap().height, 1);

        let data2 = Data {
            direction: "Export".to_string(),
            year: 2023,
            date: "01/02/2023".to_string(),
            weekday: "Monday".to_string(),
            country: "USA".to_string(),
            comodity: "Electronics".to_string(),
            transport_mode: "Sea".to_string(),
            measure: "Tons".to_string(),
            value: 2000,
            cumulative: 2000,
        };

        tree.insert(&data2);
        assert_eq!(tree.root.as_ref().unwrap().right.as_ref().unwrap().data, data2);
        assert_eq!(tree.root.as_ref().unwrap().height, 2);
    }

    #[test]
    fn test_avl_search() {
        let mut tree = AvlTree::new();
        let data = Data {
            direction: "Import".to_string(),
            year: 2023,
            date: "01/01/2023".to_string(),
            weekday: "Sunday".to_string(),
            country: "USA".to_string(),
            comodity: "Electronics".to_string(),
            transport_mode: "Air".to_string(),
            measure: "Tons".to_string(),
            value: 1000,
            cumulative: 1000,
        };
        tree.insert(&data);

        let search_result = tree.search(&data.date);
        assert!(search_result.is_some());
        assert_eq!(search_result.unwrap().data, data);

        let search_result = tree.search("nonexistent key");
        assert!(search_result.is_none());
    }

    #[test]
    fn test_avl_delete() {
        let mut tree = AvlTree::new();
        let data = Data {
            direction: "Import".to_string(),
            year: 2023,
            date: "01/01/2023".to_string(),
            weekday: "Sunday".to_string(),
            country: "USA".to_string(),
            comodity: "Electronics".to_string(),
            transport_mode: "Air".to_string(),
            measure: "Tons".to_string(),
            value: 1000,
            cumulative: 1000,
        };
         tree.insert(&data);

        tree.delete(&data.date);

        let search_result = tree.search(&data.date);
        assert!(search_result.is_none());
    }

    #[test]
    fn test_avl_edit() {
        let mut tree = AvlTree::new();
        let data = Data {
            direction: "Import".to_string(),
            year: 2023,
            date: "01/01/2023".to_string(),
            weekday: "Sunday".to_string(),
            country: "USA".to_string(),
            comodity: "Electronics".to_string(),
            transport_mode: "Air".to_string(),
            measure: "Tons".to_string(),
            value: 1000,
            cumulative: 1000,
        };
         tree.insert(&data);

        println!("Enter new value: ");

        let new_value = 5000; // Change the value for the edit operation
        tree.edit(&data.date, new_value);

        let search_result = tree.search(&data.date);
        assert!(search_result.is_some());
        assert_eq!(search_result.unwrap().data.value, new_value);
    }
    #[test]
    fn test_read_data_from_csv() {
        // For this, you would ideally have a sample CSV file for testing
        let tree = read_data("test.csv").unwrap();
        // Insert some assertions based on the data in your test CSV
        // Example:
        assert_eq!(height(&tree.root), 5); 
    }
}
