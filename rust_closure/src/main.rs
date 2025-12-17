

//task 1: Basic Closure
fn main() {
    let operation = |a: i32, b: i32| {
        // Your implementation here
        a*b
    };

    println!("Result: {}", operation(10, 5));
}

//Task 2: Environment Capture
fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        // Your implementation here
    };

    update();
    update();
}

fn main() {
    track_changes();
}


//Task 3: Vector Transformatio
fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    // Your implementation here
}

fn main() {
    let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| {
        // Implement: multiply each number by 2
    });

    let replaced = process_vector(numbers, |x| {
        // Implement: if number > 2, replace with 0, else keep number
    });

    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);
}

//Task 5: Lazy Computation 
use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    // Add fields here
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        // Your implementation here
    }

    fn get_result(&mut self) -> String {
        // Your implementation here
    }
}

fn main() {
    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());
    
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}