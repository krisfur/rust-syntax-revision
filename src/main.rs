use std::fs;
use std::io::Result; //import the ability to handle results //import the file system module

use ndarray::Array2;
use rand::Rng; //cargo add rand -> random numbers
use rand_distr::{Distribution, Normal}; //cargo add rand_distr -> random distributions //cargo add ndarray@0.15 -> 2D arrays, also has 3D etc., linfa breaks with 0.16

use rayon::join; //cargo add rayon -> divide and conquer algorithm approach
use rayon::prelude::*; //cargo add rayon -> parallelism
use std::thread; //for manual threading of full functions
use std::time::Duration; //so we can sleep set amount of time

mod fitting;
mod plotting; //import a side file as a module
use polars::prelude::*; //cargo add polars --feratures lazy,ndarray -> for dataframes

// for deep learning use burn -  tt can use tch (LibTorch), candle (a minimalist ML framework), ndarray (CPU), or wgpu (GPU via WebGPU) as its backend
// for just running inference with ONXX models use ort

// for GUI apps use egui

// for web backend use axum

// for command line arguments use clap

// for serialization and deserialization use serde
// for JSON use serde_json

// make something about tauri?
// iota example for structs?

// python bindings with pyo3

// C bindings - built in
/*
extern "C" {
    fn sqrt(x: f64) -> f64;
}
*/
// or bindgen
/*
cargo install bindgen
bindgen path/to/header.h -o bindings.rs
*/

// C++ bindings with cxx
/*
#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("my_header.h");
        fn cpp_func(x: i32) -> i32;
    }
}
*/

//------------------------------------

//basic function with error handling
pub fn load(path: &str) -> Result<String> {
    //public function, takes pointer to a string, returns String content of file or error
    let content: String = fs::read_to_string(path)?; //? at the end of the line denotes this could be an error to handle
    Ok(content)
}

//basic struct - Rust custom data type that can hold multiple values
pub struct Numberinator {
    pub number: i32,  //public field to hold a number
    pub name: String, //public field to hold a name
}

#[derive(Debug)] //this adds printing functionality
#[allow(dead_code)] //this stops warnings if not all options are used
enum Event {
    Load,                     //simply named event
    Press(char),              // Variant with a single value
    Click { x: i32, y: i32 }, // Variant with named data (like a mini-struct)
}

impl Numberinator {
    //implementation block for Numberinator struct - OOP stuff
    pub fn new(number: i32, name: String) -> Self {
        //constructor function to create a new instance of Numberinator
        Numberinator { number, name } //returns a new instance with the provided number and name
    }

    pub fn display(&self) {
        //method to display the number and name
        println!("Number: {}, Name: {}", self.number, self.name); //prints the number and name to the console
    }

    pub fn increment(&mut self) {
        //method to increment the number - mutable reference to self since it modifies the value
        self.number += 1; //increments the number by 1
    }
}

// parallelised function with divide and conquer approach
fn parallel_min(slice: &[i32]) -> i32 {
    const THRESHOLD: usize = 10;

    if slice.len() <= THRESHOLD {
        //if small enough problem: do serially
        *slice.iter().min().unwrap()
    } else {
        let mid = slice.len() / 2; //split in half
        let (left, right) = slice.split_at(mid);

        let (min_left, min_right) = join(
            //run on parts of the problem in parallel and join results
            || parallel_min(left),
            || parallel_min(right),
        );

        min_left.min(min_right)
    }
}

//normal function to multithread
fn task(name: &str) {
    let mut rng = rand::rng();
    let sleep_secs = rng.random_range(0.5..=3.0);
    println!("{name} sleeping for {sleep_secs:.2} seconds...");
    thread::sleep(Duration::from_secs_f64(sleep_secs));
    println!("{name} finished!");
}

//main function, entry point of the program
fn main() {
    println!("Hello, crusteceans 🦀!"); //prints a message to the console

    // variables, casting

    let mut x: i32 = 123; //define: mutable int32, can also infer like let x = 3, but then you miss some specific expressive functions like .abs()
    x = x * -1; //swap sign
    println!("{}", x);
    let sign = if x < 0 { -1 } else { 1 }; //find sign
    let xstr = x.abs().to_string(); //convert to string ignoring the sign
    let srev = xstr.chars().rev().collect::<String>(); //reverse the string
    println!("{}", srev);
    let irev = match srev.parse::<i32>() {
        //save new variable irev with either the string parsed back to int or 0 if it fails
        Ok(num) => num * sign, //reintroduce the sign
        Err(_) => 0,
    };
    println!("{}", irev);

    // str and String

    let xstr = "str slice"; //str, stored on a stack, immutable fixed size
    let mut x_string = String::from("proper String"); //String, stored on the heap, can be mutable and expandable (vector like in C++)

    //xstr = xstr + "a"; -> this is not valid! even if you make xstr mut it will tell you to use String instead
    println!("{}", xstr);
    x_string = x_string + " oh yeah"; //this works perfectly fine!
    println!("{}", x_string);

    //***********
    // Functions
    //***********

    println!("");
    println!("###########");
    println!("FUNCTIONS");
    println!("");

    // Load a file using defined function and handle potential errors

    // match case option - controlled flow, returns content variable or e variable
    println!("Loading file...");
    match load("example.txt") {
        //attempts to load a file named "example.txt"
        Ok(content) => println!("File content: {}", content), //if successful, prints the content of the file
        Err(e) => println!("Error loading file: {}", e),      //
    }

    // handle error using the ? operator - shorthand for simple usecases
    let content2 = load("example.txt"); //attempts to load the same file again
    println!("File content2: {:?}", content2); //prints the result, which can be Ok or Err

    // handle error using unwrap_or_else - gives a default value for the target variable
    let content3 = load("non_existent_file.txt").unwrap_or_else(|err| {
        //attempts to load a non-existent file and handles the error
        println!("Error: {}", err); //prints the error message
        String::from("Default content") //returns a default string if the file cannot be loaded
    });
    println!("File content3: {}", content3); //prints the content, which will be the default string in case of an error

    //***********
    // Structs
    //***********

    println!("");
    println!("###########");
    println!("STRUCTS");
    println!("");

    // Create an instance of the defined struct and demonstrate its functionality
    let mut numberinator = Numberinator::new(42, String::from("Initially 42.")); //creates a new instance of Numberinator with number 42 and name "Forty-Two" - mutable so we can change its values
    numberinator.display(); //calls the display method to print the number and name
    numberinator.increment(); //calls the increment method to increase the number by 1
    numberinator.display(); //calls the display method again to show the updated number and name

    //***********
    // Loops
    //***********

    println!("");
    println!("###########");
    println!("LOOPS");
    println!("");

    // Loop over a range
    for i in 0..5 {
        //loop from 0 to 4
        println!("Loop iteration: {}", i); //prints the current iteration number
        numberinator.increment(); //increments the number in Numberinator
        numberinator.display(); //displays the updated number and name
    }

    // While loop example
    let mut count = 0; //initialize a counter variable - mutable!
    while count < 5 {
        //loop while count is less than 5
        println!("While loop count: {}", count); //prints the current count
        count += 1;
    }

    //**************
    // Flow control
    //**************

    println!("");
    println!("###########");
    println!("FLOW CONTROL");
    println!("");

    // if else example
    let value = 10; //initialize a value
    if value < 5 {
        //check if value is less than 5
        println!("Value is less than 5"); //prints if true
    } else if value < 10 {
        //check if value is less than 10
        println!("Value is less than 10 but greater than or equal to 5"); //prints if true
    } else {
        println!("Value is 10 or greater"); //prints if none of the above conditions are true
    }

    // match example
    let number = 3; //initialize a number
    match number {
        //match statement to check the value of number
        1 => println!("Number is one"), //if number is 1, prints this
        2 => println!("Number is two"), //if number is 2, prints this
        3 => println!("Number is three"), //if number is 3, prints this
        _ => println!("Number is something else"), //if number is anything else, prints this
    }

    //**************
    // RNG
    //**************

    println!("");
    println!("###########");
    println!("RNG");
    println!("");

    // Just RNG
    let mut rng = rand::rng(); //instantiate random numer generator
    let num: u32 = rng.random_range(0..=10); // inclusive range 0 to 10
    println!("Random number between 0 and 10: {}", num);

    // RNG sample from normal distribution
    let normal = Normal::new(0.0, 1.0).unwrap(); // mean = 0, std dev = 1
    let mut rng2 = rand::rng();
    let sample = normal.sample(&mut rng2);
    println!("Random sample from N(0,1): {}", sample);

    // RNG distribution into an ndarray::Array2
    let mut rng3 = rand::rng();
    let normal = Normal::new(0.0, 1.0).unwrap(); // mean = 0, std dev = 1
    let rows = 3;
    let cols = 4;
    // Generate a 2D array of normal-distributed samples
    let data: Vec<f64> = (0..rows * cols).map(|_| normal.sample(&mut rng3)).collect();

    let array: Array2<f64> = ndarray::Array2::from_shape_vec((rows, cols), data).unwrap();
    println!("Generated normal-distributed array:\n{array}");

    //**************
    // Arrays
    //**************

    println!("");
    println!("###########");
    println!("Arrays");
    println!("");

    // basic array - note the predefined length!
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];

    // access data
    println!("First: {}", arr[0]);

    // modify - needs mut!
    arr[2] = 99;

    // iterate
    for val in arr.iter() {
        println!("{}", val);
    }

    // expressive way to get sum
    let arr: [f64; 3] = [1.1, 2.2, 3.3];
    let sum: f64 = arr.iter().sum();
    println!("Sum = {}", sum);

    // strings
    let greetings: [&str; 3] = ["hello", "hi", "hola"];
    for s in &greetings {
        println!("Greeting: {}", s);
    }

    // 2d array - matrix
    let mut a = Array2::<f64>::zeros((3, 3));

    // Set diagonal values to 1
    for i in 0..3 {
        a[(i, i)] = 1.0;
    }

    println!("Identity matrix:\n{}", a);

    // Vector - array of mutable length

    let mut v: Vec<i32> = Vec::new(); // empty vector of i32

    // Add values
    v.push(10);
    v.push(20);
    v.push(30);
    println!("After pushes: {:?}", v);

    // Remove last value
    if let Some(val) = v.pop() {
        println!("Popped value: {}", val);
    }

    println!("After pop: {:?}", v);

    for val in &v {
        //take reverence when iterating over a vector!
        println!("Value: {}", val);
    }

    // vector with strings
    let mut names: Vec<String> = vec![];

    names.push("Alice".to_string());
    names.push("Bob".into());

    println!("{:?}", names);

    names.pop();
    println!("{:?}", names);

    //**************
    // Parallelism/Threads
    //**************

    println!("");
    println!("###########");
    println!("Parallelism/Threads");
    println!("");

    // Rayon - for applying on collections, slices, ranges, vectors, etc.
    let data = vec![1, 2, 3, 4, 5];
    println!("Data: {:?}", &data);
    println!("Performing x * x in parallel.");
    let results: Vec<_> = data.par_iter().map(|x| x * x).collect();
    println!("Results: {:?}", &results);

    let mut v = vec![5, 3, 8, 1, 2];
    println!("Vector to sort in parallel: {:?}", &v);
    v.par_sort_unstable(); // uses multiple threads
    println!("Sorted: {:?}", v);

    let sum: i32 = (1..=100).into_par_iter().sum(); //this can thread panic if you exceed max stack size!
    println!("Parallel sum: {}", sum);

    let max = (1..=100).into_par_iter().max().unwrap();
    println!("Parallel max: {}", max);

    let data: Vec<i32> = (1..=100).rev().collect();
    println!("Vector to find the minimum of in parallel: {:?}", &data);
    let min_value = parallel_min(&data); //splits too large a task into subtasks that run in parallel, and joins the results
    println!("Min value: {}", min_value);

    // std::thread - for manually parallelising running functions
    let handle1 = thread::spawn(|| task("Task 1"));
    let handle2 = thread::spawn(|| task("Task 2"));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Both tasks complete.");

    //**************
    // DataFrames, clustering, plotting, fitting
    //**************

    println!("");
    println!("###########");
    println!("DataFrames, fitting, plotting");
    println!("");

    // clustering
    let x = vec![1.0, 1.1, 1.2, 5.0, 5.1, 5.2, 9.0, 9.1, 9.2];
    let y = vec![1.0, 0.9, 1.1, 5.0, 5.2, 5.1, 9.0, 9.2, 8.9];

    let df = match df![ //make it a DF
        "x" => &x,
        "y" => &y
    ] {
        Ok(df) => df,
        Err(e) => {
            eprintln!("Error creating DataFrame: {e}");
            return;
        }
    };

    if let Err(e) = plotting::plot_dataframe(&df) {
        //pulling from a side module mod plotting;
        eprintln!("❌ Plotting failed: {e}");
    }

    // fitting
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let y = vec![1.1, 1.9, 3.2, 4.1, 5.0, 6.5, 6.9, 7.6, 8.8];

    let df = df![
        "x" => &x,
        "y" => &y
    ]
    .unwrap();

    if let Err(e) = fitting::fit_and_plot(&df) {
        eprintln!("❌ Fitting failed: {e}");
    }

    //*******************
    // Enums vs Structs
    //*******************

    println!("");
    println!("###########");
    println!("Enums vs Structs");
    println!("");

    // In Rust structs are used to make custom data types with named fields.
    // Enums, on the other hand, are used to represent a fixed set of possible values.
    // Structs -> AND
    // Enums -> OR

    let key_press_event = Event::Press('q');
    let click_event = Event::Click { x: 100, y: 250 };

    println!("Key press event: {:?}", key_press_event);
    println!("Click event: {:?}", click_event);
}
