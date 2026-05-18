#![warn(clippy::all, clippy::pedantic)]

pub fn bubble_sort(arr: &mut [i32], ascending: bool) {

    println!("Array input: {arr:?}");

    let mut steps: usize = 0;

    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        
        for j in 0..n - i - 1 {
            if ascending {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                    swapped = true;
                }
            } else if arr[j] < arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
            steps += 1;
        }
        
        // Если не было обменов - массив отсортирован
        if !swapped {
            break;
        }
    }
    println!("Array output: {arr:?}");
    println!("Steps used: {}", steps);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort_ascending() {
        let mut arr = [4, 3, 5, 1, 2];
        bubble_sort(&mut arr, true);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn test_bubble_sort_descending() {
        let mut arr = [4, 3, 5, 1, 2];
        bubble_sort(&mut arr, false);
        assert_eq!(arr, [5, 4, 3, 2, 1]);
    }
    
    #[test]
    fn test_bubble_sort_already_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        bubble_sort(&mut arr, true);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn test_bubble_sort_empty() {
        let mut arr: [i32; 0] = [];
        bubble_sort(&mut arr, true);
        assert_eq!(arr, []);
    }
    
    #[test]
    fn test_bubble_sort_single_element() {
        let mut arr = [42];
        bubble_sort(&mut arr, true);
        assert_eq!(arr, [42]);
    }
}

