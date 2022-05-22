fn main() {
    println!("Hello, world!");

    let mut v = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("The average of {:?} is {}", v, average(&v));

    // wasn't able to show the array itself, with the median
    // got immutable + mutable borrow error
    println!("The median of the above array is {}", median(&mut v));
}

// given a list of integers, use a vector and return the mean
fn average(numbers: &[i32]) -> f64 {
    let sum: i32 = numbers.iter().sum();

    sum as f64 / numbers.len() as f64
}

fn median(numbers: &mut [i32]) -> f64 {
    numbers.sort();
    let size = numbers.len();
    let mid = size / 2;

    if size % 2 == 0 {
        return average(&[numbers[mid-1], numbers[mid]]);
    } else {
        return numbers[mid] as f64;
    }
}