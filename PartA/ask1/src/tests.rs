use crate::Data;
use crate::counting_sort;
use crate::merge_sort_par;
use crate::read_data;

#[cfg(test)]
mod ask1 {
    use super::*;

    #[tokio::test]
    async fn test_counting_sort() {
        let nums = vec![1, 4, 2, 0, 3];
        let mut data = vec![Data::new(); nums.len()];
        for (i, d) in data.iter_mut().enumerate() {
            d.value = nums[i] as u64;
        }
        counting_sort(&mut data);
        for (i, d) in data.iter().enumerate() {
            assert_eq!(d.value, i as u64);
        }
    }

    #[test]
    fn test_merge_sort() {
        let nums = vec![1, 4, 2, 0, 3];
        let mut data = vec![Data::new(); nums.len()];
        for (i, d) in data.iter_mut().enumerate() {
            d.value = nums[i] as u64;
        }
        merge_sort_par(&mut data);
        for (i, d) in data.iter().enumerate() {
            assert_eq!(d.value, i as u64);
        }
    }

    #[test]
    fn test_read_data_from_csv(){
        let data = read_data("test.csv");
        assert_eq!(data.len(), 18);
    }
}
