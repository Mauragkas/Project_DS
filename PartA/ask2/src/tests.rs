use crate::Data;
use crate::heap_sort;
use crate::quick_sort_par;

#[cfg(test)]
mod ask2 {

    use super::*;

    #[test]
    fn test_heap_sort() {
        let nums = vec![1, 4, 2, 0, 3];
        let mut data = vec![Data::new(); nums.len()];
        for (i, d) in data.iter_mut().enumerate() {
            d.cumulative = nums[i] as u64;
        }
        heap_sort(&mut data);
        for (i, d) in data.iter().enumerate() {
            assert_eq!(d.cumulative, i as u64);
        }
    }

    #[test]
    fn test_quick_sort_par() {
        let nums = vec![1, 4, 2, 0, 3];
        let mut data = vec![Data::new(); nums.len()];
        for (i, d) in data.iter_mut().enumerate() {
            d.cumulative = nums[i] as u64;
        }
        let len = data.len();
        quick_sort_par(&mut data);
        for (i, d) in data.iter().enumerate() {
            assert_eq!(d.cumulative, i as u64);
        }
    }

}