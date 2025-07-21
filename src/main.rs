use std::io::Result; //import the ability to handle results
use std::fs; //import the file system module

use rand::Rng; //cargo add rand
use rand_distr::{Normal, Distribution}; //cargo add rand_distr
use ndarray::Array2; //cargo add ndarray

// for dates and times use chrono

// for HTTP requests use reqwest

// for multithreading use std::thread

// for async programming use tokio

// for command line arguments use std::env or clap
// for environment variables use std::env
// for logging use log

// for serialization and deserialization use serde
// for JSON use serde_json
// for dataframes use polars
// for plotting use plotters
// for clustering and such use linfa

// make something about tauri?
// iota example for structs?

//------------------------------------

//basic function with error handling
pub fn load(path: &str) -> Result<String> { //public function, takes pointer to a string, returns String content of file or error
    let content: String = fs::read_to_string(path)?; //? at the end of the line denotes this could be an error to handle
    Ok(content)
}

//basic struct - Rust custom data type that can hold multiple values
pub struct Numberinator{
    pub number: i32, //public field to hold a number
    pub name: String, //public field to hold a name
}

impl Numberinator { //implementation block for Numberinator struct - OOP stuff
    pub fn new(number: i32, name: String) -> Self { //constructor function to create a new instance of Numberinator
        Numberinator { number, name } //returns a new instance with the provided number and name
    }

    pub fn display(&self) { //method to display the number and name
        println!("Number: {}, Name: {}", self.number, self.name); //prints the number and name to the console
    }

    pub fn increment(&mut self) { //method to increment the number - mutable reference to self since it modifies the value
        self.number += 1; //increments the number by 1
    }
}

 //main function, entry point of the program
fn main() {
    println!("Hello, crusteceans 🦀!"); //prints a message to the console

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
    match load("example.txt") { //attempts to load a file named "example.txt"
        Ok(content) => println!("File content: {}", content), //if successful, prints the content of the file
        Err(e) => println!("Error loading file: {}", e), //
        }

    // handle error using the ? operator - shorthand for simple usecases
    let content2 = load("example.txt"); //attempts to load the same file again
    println!("File content2: {:?}", content2); //prints the result, which can be Ok or Err

    // handle error using unwrap_or_else - gives a default value for the target variable
    let content3 = load("non_existent_file.txt").unwrap_or_else(|err| { //attempts to load a non-existent file and handles the error
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
    let mut numberinator = Numberinator::new(42, String::from("Initially 42.")); //creates a new instance of Numberinator with number 42 and name "Forty-Two"
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
    for i in 0..5 { //loop from 0 to 4
        println!("Loop iteration: {}", i); //prints the current iteration number
        numberinator.increment(); //increments the number in Numberinator
        numberinator.display(); //displays the updated number and name
    }

    // While loop example
    let mut count = 0; //initialize a counter variable
    while count < 5 { //loop while count is less than 5
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
    if value < 5 { //check if value is less than 5
        println!("Value is less than 5"); //prints if true
    } else if value < 10 { //check if value is less than 10
        println!("Value is less than 10 but greater than or equal to 5"); //prints if true
    } else {
        println!("Value is 10 or greater"); //prints if none of the above conditions are true
    }

    // match example
    let number = 3; //initialize a number
    match number { //match statement to check the value of number
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
    let data: Vec<f64> = (0..rows * cols)
        .map(|_| normal.sample(&mut rng3))
        .collect();

    let array: Array2<f64> = ndarray::Array2::from_shape_vec((rows, cols), data).unwrap();
    println!("Generated normal-distributed array:\n{array}");

    //**************
    // Arrays
    //**************

    println!("");
    println!("###########");
    println!("Arrays");
    println!("");

    // add various array manipulation stuff here

    //**************
    // Parallelism/Threads
    //**************

    println!("");
    println!("###########");
    println!("Parallelism/Threads");
    println!("");

    //do parallelised computing
}
