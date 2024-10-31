use add::add;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut z=0;
    for i in 1..100000 {
        z=add(&z,&i);
    }
    println!("The addition of all numbers between 1 and 100,000 is {}", z);
    let duration = start.elapsed();
    println!(
        "Time taken by get_sum: {} microseconds",
        duration.as_micros())
}
