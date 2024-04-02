use std::io;

fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if arr[mid] == target {
            // Check if this is the first occurrence
            if mid == 0 || arr[mid - 1] != target {
                return Some(mid);
            } else {
                right = mid - 1; // Continue searching in the left half
            }
        } else if arr[mid] < target {
            left = mid + 1; // Continue searching in the right half
        } else {
            right = mid - 1; // Continue searching in the left half
        }
    }

    None // Element not found
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

    // Read the target number from the user
    println!("Enter the number to search for:");
    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("Failed to read line");
    let target: i32 = target.trim().parse().expect("Invalid number");

    // Find the index of the first occurrence of the target number
    match find_first_occurrence(&arr, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} not found in the array.", target),
    }
}
