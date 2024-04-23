fn max_subarray_sum(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0; // Empty array case
    }

    let mut max_sum = arr[0];
    let mut current_sum = arr[0];

    for &num in &arr[1..] {
        current_sum = current_sum.max(num);
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}

fn main() {
    let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = max_subarray_sum(&arr);
    println!("Maximum subarray sum: {}", max_sum);
}
