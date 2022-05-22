fn main() {
    println!("Hello, world!");

    let v = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("The average of {:?} is {}", v, average(&v));
}

// given a list of integers, use a vector and return the mean
fn average(numbers: &[i32]) -> f64 {
    let sum: i32 = numbers.iter().sum();

    sum as f64 / numbers.len() as f64
}

