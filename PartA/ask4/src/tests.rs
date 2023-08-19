#![allow(unused)]

#[cfg(test)]

use crate::Data;
use crate::date_to_days;
use crate::read_data;
use crate::bis;
use crate::in_range;

mod tests {
    use super::*;

    #[test]
    fn test_date_to_days() {
        assert_eq!(date_to_days("01/01/2020"), 737331);
        assert_eq!(date_to_days("31/12/2020"), 737691);
    }

    #[test]
    fn test_in_range() {
        let data = vec![
            Data {
                direction: "Import".to_string(),
                year: 2019,
                date: "01/01/2019".to_string(),
                weekday: "Tuesday".to_string(),
                country: "USA".to_string(),
                comodity: "Oil".to_string(),
                transport_mode: "Ship".to_string(),
                measure: "Tons".to_string(),
                value: 10000,
                cumulative: 10000,
            },
            Data {
                direction: "Export".to_string(),
                year: 2020,
                date: "31/12/2020".to_string(),
                weekday: "Thursday".to_string(),
                country: "China".to_string(),
                comodity: "Coal".to_string(),
                transport_mode: "Train".to_string(),
                measure: "Tons".to_string(),
                value: 20000,
                cumulative: 30000,
            }
        ];

        assert_eq!(in_range(&data, "01/01/2019"), true);
        assert_eq!(in_range(&data, "01/01/2021"), false);
    }

    // assuming that bis function will return first index when date is 01/01/2019
    #[test]
    fn test_bis() {
        let data = vec![
            Data {
                direction: "Import".to_string(),
                year: 2019,
                date: "01/01/2019".to_string(),
                weekday: "Tuesday".to_string(),
                country: "USA".to_string(),
                comodity: "Oil".to_string(),
                transport_mode: "Ship".to_string(),
                measure: "Tons".to_string(),
                value: 10000,
                cumulative: 10000,
            },
            Data {
                direction: "Export".to_string(),
                year: 2020,
                date: "31/12/2020".to_string(),
                weekday: "Thursday".to_string(),
                country: "China".to_string(),
                comodity: "Coal".to_string(),
                transport_mode: "Train".to_string(),
                measure: "Tons".to_string(),
                value: 20000,
                cumulative: 30000,
            }
        ];

        assert_eq!(bis(&data, "01/01/2019"), (true, 0));
        assert_eq!(bis(&data, "01/01/2021"), (false, 0));
    }
}
