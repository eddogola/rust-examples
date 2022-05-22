use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut v = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 5];
    println!("The average of {:?} is {}", v, average(&v));

    // wasn't able to show the array itself, with the median
    // got immutable + mutable borrow error
    println!("The median of the above array is {}", median(&mut v));

    println!("The mode of {:?} is {:?}", v, mode(&v));
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

fn mode(numbers: &[i32]) -> i32 {
    let mut map = HashMap::new();

    for num in numbers {
        map.entry(num).or_insert(0) += 1;
    }

    // TODO
    // get most occuring number by count - map value
}