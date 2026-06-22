use std::io;

mod utilities; // Объявляем модули на уровне крейта
mod temp_converter;
mod guess_game; 

mod binary_search;
mod bubble_sorting;

mod calculator;

mod stack;
mod queue;

mod counter;
mod hexagon_2D;
mod hexagon_3D;
mod hexagon_3D_v2; 



fn main() {
    //====================================//
    //                 #1                 //
    //====================================//
    // guess_game::start_game();
    // temp_converter::temp_converter();


    //====================================//
    //                 #2                 //
    //====================================//
    //let arr1 = [-1, 2, 3, 5, 7, 8, 10, 24, 37, 42, 135];
    //binary_search::bin_search(&arr1, 37);


    //let mut arr2 = [4, 3, 5, 1, 2];
    //bubble_sorting::bubble_sort(&mut arr2, true);
    //bubble_sorting::bubble_sort(&mut arr2, false);


    //====================================//
    //                 #3                 //
    //====================================//
    // calculator::run_calculator();


    //====================================//
    //                 #4                 //
    //====================================//
    /* 
    let bracketer = stack::Bracketer::new();
    let test_string = String::from("[hello {World} (test");
    if bracketer.check(test_string.as_str()) {
        println!("Строка '{}' корректна", test_string);
    } else {
        println!("Строка '{}' некорректна", test_string);
    }
    */
    
    /* 
    print!("Введите число: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice).unwrap();

    let n_choice = utilities::str_to_num(&user_choice); // borrow!! the string and convert to f64
    println!("Number: {}", n_choice);
    println!("String: {}", user_choice);
    */

    /* 
    let mut bg = queue::BackgroundTask::new();
    bg.add("job 1");
    bg.add("job 2");
    bg.add("job 3");
    bg.execute();
    */

    // counter::run_counter();
    // hexagon_2D::run_polygon();
    // hexagon_3D::run_polygon();
    hexagon_3D_v2::run_3d_parametric();

}