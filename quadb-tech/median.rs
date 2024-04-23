fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        // For an even number of elements, average the middle two
        let mid_left = arr[len / 2 - 1] as f64;
        let mid_right = arr[len / 2] as f64;
        (mid_left + mid_right) / 2.0
    } else {
        // For an odd number of elements, return the middle element
        arr[len / 2] as f64
    }
}

fn main() {
    let arr_even = [1, 2, 3, 4, 5, 6];
    let arr_odd = [1, 2, 3, 4, 5];

    println!("Median of {:?}: {}", arr_even, find_median(&arr_even));
    println!("Median of {:?}: {}", arr_odd, find_median(&arr_odd));
}
