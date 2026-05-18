//use std::io;
mod utilities; // Объявляем модули на уровне крейта
mod temp_converter;
mod guess_game; 

mod binary_search;
mod bubble_sorting;


fn main() {
    // #1
    // guess_game::start_game();
    // temp_converter::temp_converter();


    // #2
    let arr1 = [-1, 2, 3, 5, 7, 8, 10, 24, 37, 42, 135];
    binary_search::bin_search(&arr1, 37);


    let mut arr2 = [4, 3, 5, 1, 2];
    bubble_sorting::bubble_sort(&mut arr2, true);
    bubble_sorting::bubble_sort(&mut arr2, false);


}