use std::io;

fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 1 {
        return arr[len / 2] as f64;
    } else {
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        return (arr[mid_left] as f64 + arr[mid_right] as f64) / 2.0;
    }
}

fn main() {
    // Read the array from the user
    println!("Enter a sorted array of integers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    // Find the median of the array
    let median = find_median(&arr);
    println!("The median of the array is: {}", median);
}
