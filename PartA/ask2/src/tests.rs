#![allow(unused)]
use crate::Data;
use crate::heap_sort;
use crate::quick_sort;

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
    fn test_quick_sort() {
        let nums = vec![1, 4, 2, 0, 3];
        let mut data = vec![Data::new(); nums.len()];
        for (i, d) in data.iter_mut().enumerate() {
            d.cumulative = nums[i] as u64;
        }
        let len = data.len();
        quick_sort(&mut data, 0, len - 1);
        for (i, d) in data.iter().enumerate() {
            assert_eq!(d.cumulative, i as u64);
        }
    }

}