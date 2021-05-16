use collection_exercises::basic_calculations::{
    CalcResults,
    calculate_median_with_sort,
    calculate_mean,
    calculate_mode,
};

fn main() {
    let mut nums: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 1];

    println!("{:?}\n", nums);

    let results = CalcResults {
        median: calculate_median_with_sort(&mut nums),
        mean: calculate_mean(&nums),
        mode: calculate_mode(&nums),
    };

    println!("{:#?}", results);
}
