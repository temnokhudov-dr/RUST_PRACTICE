#![warn(clippy::all, clippy::pedantic)]

use std::cmp::Ordering;

pub fn bin_search(arr: &[i32], desired_value: i32) -> Option<(i32, usize)> {
    let mut low_bound: usize = 0;
    let mut up_bound: usize = arr.len() - 1;
    let mut i: usize = 0;

    while low_bound <= up_bound {
        i += 1;

        let mid = low_bound + (up_bound - low_bound) / 2;

        let mid_value = arr[mid];

        /* 
        if mid_value == desired_value {
            println!("Found value {mid_value} at {mid}");
            return Some((mid_value, mid));
        } else if desired_value > mid_value {
            low_bound = mid + 1;
        } else {
            up_bound = mid - 1;
        }
        */

        match mid_value.cmp(&desired_value) {
            Ordering::Equal => {
                println!("Found value {mid_value} at {mid}");
                return Some((mid_value, mid));
            },
            Ordering::Greater => {
                up_bound = match mid.checked_sub(1) { // безопасно отнимаем 1 от usize 
                    Some(result) => result,
                    _ => {
                        println!("Not found!");
                        return None
                    }
                }
            },
            Ordering::Less => low_bound = mid + 1,
        }

        println!("Step {i}");
    }
    println!("Not found!");
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    const ARR: [i32; 10] = [-1, 2, 3, 5, 7, 8, 10, 24, 37, 42];

    #[test]
    fn element_found() {
        assert_eq!((-1, 0), bin_search(&ARR, -1).unwrap());
    }

    #[test]
    fn element_not_found() {
        let result = bin_search(&ARR, 99);
        assert!(result.is_none());
    }

    #[test]
    fn smallest_element_not_found() {
        let result = bin_search(&ARR, -100);
        assert!(result.is_none());
    }
}