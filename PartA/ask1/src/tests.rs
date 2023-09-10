use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_sort_date() {
        let mut data = vec![
            Data {
                direction: "Import".to_string(),
                year: 2023,
                date: "15/03/2023".to_string(),
                weekday: "Wednesday".to_string(),
                country: "USA".to_string(),
                comodity: "Electronics".to_string(),
                transport_mode: "Air".to_string(),
                measure: "Units".to_string(),
                value: 45,
                cumulative: 150,
            },
            Data {
                direction: "Export".to_string(),
                year: 2023,
                date: "10/01/2023".to_string(),
                weekday: "Tuesday".to_string(),
                country: "UK".to_string(),
                comodity: "Cars".to_string(),
                transport_mode: "Sea".to_string(),
                measure: "Units".to_string(),
                value: 20,
                cumulative: 50,
            },
            Data {
                direction: "Import".to_string(),
                year: 2023,
                date: "02/02/2023".to_string(),
                weekday: "Thursday".to_string(),
                country: "Canada".to_string(),
                comodity: "Machinery".to_string(),
                transport_mode: "Train".to_string(),
                measure: "Units".to_string(),
                value: 30,
                cumulative: 100,
            },
        ];

        counting_sort(&mut data);

        assert_eq!(data[0].date, "10/01/2023");
        assert_eq!(data[1].date, "02/02/2023");
        assert_eq!(data[2].date, "15/03/2023");
    }

    #[test]
    fn test_counting_sort_value() {
        let mut data = vec![
            Data {
                direction: "Import".to_string(),
                year: 2023,
                date: "02/02/2023".to_string(),
                weekday: "Wednesday".to_string(),
                country: "USA".to_string(),
                comodity: "Electronics".to_string(),
                transport_mode: "Air".to_string(),
                measure: "Units".to_string(),
                value: 1,
                cumulative: 150,
            },
            Data {
                direction: "Export".to_string(),
                year: 2023,
                date: "02/02/2023".to_string(),
                weekday: "Tuesday".to_string(),
                country: "UK".to_string(),
                comodity: "Cars".to_string(),
                transport_mode: "Sea".to_string(),
                measure: "Units".to_string(),
                value: 2,
                cumulative: 50,
            },
            Data {
                direction: "Import".to_string(),
                year: 2023,
                date: "02/02/2023".to_string(),
                weekday: "Thursday".to_string(),
                country: "Canada".to_string(),
                comodity: "Machinery".to_string(),
                transport_mode: "Train".to_string(),
                measure: "Units".to_string(),
                value: 3,
                cumulative: 100,
            },
        ];

        counting_sort(&mut data);

        assert_eq!(data[0].value, 1);
        assert_eq!(data[1].value, 2);
        assert_eq!(data[2].value, 3); 
    }

    #[test]
    fn test_merge_sort_par_date() {
        let mut data = vec![
            Data {
                direction: "Import".to_string(),
                year: 2023,
                date: "15/03/2023".to_string(),
                weekday: "Wednesday".to_string(),
                country: "USA".to_string(),
                comodity: "Electronics".to_string(),
                transport_mode: "Air".to_string(),
                measure: "Units".to_string(),
                value: 45,
                cumulative: 150,
            },
            Data {
                direction: "Export".to_string(),
                year: 2023,
                date: "10/01/2023".to_string(),
                weekday: "Tuesday".to_string(),
                country: "UK".to_string(),
                comodity: "Cars".to_string(),
                transport_mode: "Sea".to_string(),
                measure: "Units".to_string(),
                value: 20,
                cumulative: 50,
            },
            Data {
                direction: "Import".to_string(),
                year: 2023,
                date: "02/02/2023".to_string(),
                weekday: "Thursday".to_string(),
                country: "Canada".to_string(),
                comodity: "Machinery".to_string(),
                transport_mode: "Train".to_string(),
                measure: "Units".to_string(),
                value: 30,
                cumulative: 100,
            },
        ];

        let mut buffer = vec![Data::new(); data.len()];
        merge_sort_par(&mut data, &mut buffer);

        assert_eq!(data[0].date, "10/01/2023");
        assert_eq!(data[1].date, "02/02/2023");
        assert_eq!(data[2].date, "15/03/2023");
    }

    #[test]
    fn test_merge_sort_par_value(){
        let mut data = vec![
            Data {
                direction: "Import".to_string(),
                year: 2023,
                date: "02/02/2023".to_string(),
                weekday: "Wednesday".to_string(),
                country: "USA".to_string(),
                comodity: "Electronics".to_string(),
                transport_mode: "Air".to_string(),
                measure: "Units".to_string(),
                value: 1,
                cumulative: 150,
            },
            Data {
                direction: "Export".to_string(),
                year: 2023,
                date: "02/02/2023".to_string(),
                weekday: "Tuesday".to_string(),
                country: "UK".to_string(),
                comodity: "Cars".to_string(),
                transport_mode: "Sea".to_string(),
                measure: "Units".to_string(),
                value: 2,
                cumulative: 50,
            },
            Data {
                direction: "Import".to_string(),
                year: 2023,
                date: "02/02/2023".to_string(),
                weekday: "Thursday".to_string(),
                country: "Canada".to_string(),
                comodity: "Machinery".to_string(),
                transport_mode: "Train".to_string(),
                measure: "Units".to_string(),
                value: 3,
                cumulative: 100,
            },
        ];

        let mut buffer = vec![Data::new(); data.len()];
        merge_sort_par(&mut data, &mut buffer);

        assert_eq!(data[0].value, 1);
        assert_eq!(data[1].value, 2);
        assert_eq!(data[2].value, 3); 
    }

    #[test]
    fn test_read_data_from_csv(){
        let data = read_data("test.csv");
        assert_eq!(data.len(), 18);
    }
}
