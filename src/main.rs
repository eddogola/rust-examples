fn main() {
    println!("Hello, world!");

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("The average of {:?} is {}", v, average(&v));
}

// given a list of integers, use a vector and return the mean
fn average(numbers: &Vec<i32>) -> f64 {
    let mut sum = 0f64;
    for num in numbers {
        sum += *num as f64;
    }

    sum / (numbers.len() as f64)
}
