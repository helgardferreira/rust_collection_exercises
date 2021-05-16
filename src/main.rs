use collection_exercises::basic_calculations::{
    CalcResults,
    calculate_median_with_sort,
    calculate_mean,
    calculate_mode,
};

use collection_exercises::pig_latin::{
    crappy_pig_latin,
    cleaner_pig_latin,
};

fn main() {
    // Basic Calculations Exercise
    let mut nums: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 1];

    println!("{:?}\n", nums);

    let results = CalcResults {
        median: calculate_median_with_sort(&mut nums),
        mean: calculate_mean(&nums),
        mode: calculate_mode(&nums),
    };

    println!("{:#?}", results);

    // Pig Latin Exercise
    let text = String::from("first-eth is worst-eth, second is best, third is durst...");

    let result = crappy_pig_latin(&text);
    let result2 = cleaner_pig_latin(&text);

    println!("{}", result);
    println!("{}", result2);
}
