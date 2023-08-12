#![allow(unused)]
use crate::Data;
use crate::hash;
use crate::init;
use crate::insert;
use crate::search;
use crate::edit;
use crate::delete;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("2023-08-13"), hash("2023-08-13"));
        assert_ne!(hash("2023-08-13"), hash("2023-08-14"));
    }

    #[test]
    fn test_insert_and_search() {
        let mut hash_table = init();
        let data = Data {
            direction: "Import".to_string(),
            year: 2023,
            date: "2023-08-13".to_string(),
            weekday: "Monday".to_string(),
            country: "CountryX".to_string(),
            comodity: "Oil".to_string(),
            transport_mode: "Sea".to_string(),
            measure: "Barrel".to_string(),
            value: 1000,
            cumulative: 50000,
        };

        insert(&mut hash_table, data.clone());

        let found_node = search(&hash_table, "2023-08-13").unwrap();
        assert_eq!(found_node.data.date, "2023-08-13");
        assert_eq!(found_node.data.value, 1000);
    }

    #[test]
    fn test_edit() {
        let mut hash_table = init();
        let data = Data {
            direction: "Import".to_string(),
            year: 2023,
            date: "2023-08-13".to_string(),
            weekday: "Monday".to_string(),
            country: "CountryX".to_string(),
            comodity: "Oil".to_string(),
            transport_mode: "Sea".to_string(),
            measure: "Barrel".to_string(),
            value: 1000,
            cumulative: 50000,
        };

        insert(&mut hash_table, data.clone());

        let mut new_data = data.clone();
        new_data.value = 1500;
        edit(&mut hash_table, "2023-08-13", new_data.clone());

        let found_node = search(&hash_table, "2023-08-13").unwrap();
        assert_eq!(found_node.data.value, 1500);
    }

    #[test]
    fn test_delete() {
        let mut hash_table = init();
        let data = Data {
            direction: "Import".to_string(),
            year: 2023,
            date: "2023-08-13".to_string(),
            weekday: "Monday".to_string(),
            country: "CountryX".to_string(),
            comodity: "Oil".to_string(),
            transport_mode: "Sea".to_string(),
            measure: "Barrel".to_string(),
            value: 1000,
            cumulative: 50000,
        };

        insert(&mut hash_table, data.clone());

        let found_node = search(&hash_table, "2023-08-13").unwrap();
        assert_eq!(found_node.data.date, "2023-08-13");

        delete(&mut hash_table, "2023-08-13");
        let found_node_after_delete = search(&hash_table, "2023-08-13");
        assert_eq!(found_node_after_delete.is_none(), true);
    }
}
