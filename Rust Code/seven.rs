use std::io;

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;
    for j in low..high {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}

fn kth_smallest(arr: &mut [i32], k: usize) -> Option<i32> {
    let len = arr.len();
    if k > 0 && k <= len {
        let mut left = 0;
        let mut right = len - 1;
        while left <= right {
            let pivot_index = partition(arr, left, right);
            if pivot_index == k - 1 {
                return Some(arr[pivot_index]);
            } else if pivot_index < k - 1 {
                left = pivot_index + 1;
            } else {
                right = pivot_index - 1;
            }
        }
    }
    None
}

fn main() {
    // Read the array from the user
    println!("Enter integers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    // Read the value of k from the user
    println!("Enter the value of k:");
    let mut input_k = String::new();
    io::stdin().read_line(&mut input_k).expect("Failed to read line");
    let k: usize = input_k.trim().parse().expect("Invalid number");

    // Find the kth smallest element
    if let Some(kth_smallest_element) = kth_smallest(&mut arr.clone(), k) {
        println!("The {}th smallest element is: {}", k, kth_smallest_element);
    } else {
        println!("Invalid value of k or array is empty.");
    }
}
