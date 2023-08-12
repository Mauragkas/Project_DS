#![allow(unused)]
use crate::Data;
use crate::convert_date_to_days;
use crate::read_data;
use crate::binary_search;
use crate::interpolation_search;
use crate::in_range;

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_convert_date_to_days() {
        assert_eq!(convert_date_to_days("01/01/2020"), 737331);
        assert_eq!(convert_date_to_days("31/12/2020"), 737691);
    }

    #[test]
    fn test_read_data() {
        let data = read_data("cs.csv");
        assert_eq!(data.len(), 111438); // assuming cs.csv has 10 records
        assert_eq!(data[0].date, "01/01/2015"); // assuming first record date is 01/01/2020
    }

    #[test]
    fn test_binary_search() {
        let data = read_data("cs.csv");
        let index = binary_search(&data, 0, data.len() - 1, convert_date_to_days("01/01/2020"));
        assert_eq!(data[index].date, "01/01/2020");
    }

    #[test]
    fn test_interpolation_search() {
        let data = read_data("cs.csv");
        let index = interpolation_search(&data, 0, data.len() - 1, convert_date_to_days("31/12/2020"));
        assert_eq!(data[index].date, "31/12/2020");
    }

    #[test]
    fn test_in_range() {
        let data = read_data("cs.csv");
        assert_eq!(in_range(&data, "01/01/2010"), false);
        assert_eq!(in_range(&data, "01/01/2020"), true);
    }
}
