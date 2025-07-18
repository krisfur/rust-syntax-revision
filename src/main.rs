use std::io::Result; //import the ability to handle results
use std::fs; //import the file system module

// for arrays use ndarray
// for random numbers use rand::thread_rng and rand_distr
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

//basic function with error handling
pub fn load(path: &str) -> Result<String> { //public function, takes pointer to a string, returns String content of file or error
    let content: String = fs::read_to_string(path)?;
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
    println!("Hello, world!"); //prints a message to the console

    // Load a file using defined function and handle potential errors
    println!("Loading file...");
    match load("example.txt") { //attempts to load a file named "example.txt"
        Ok(content) => println!("File content: {}", content), //if successful, prints the content of the file
        Err(e) => println!("Error loading file: {}", e), //
        }

    // Create an instance of the defined struct and demonstrate its functionality
    let mut numberinator = Numberinator::new(42, String::from("Initially 42.")); //creates a new instance of Numberinator with number 42 and name "Forty-Two"
    numberinator.display(); //calls the display method to print the number and name
    numberinator.increment(); //calls the increment method to increase the number by 1
    numberinator.display(); //calls the display method again to show the updated number and name

    // Loop over a range
    for i in 0..5 { //loop from 0 to 4
        println!("Loop iteration: {}", i); //prints the current iteration number
        numberinator.increment(); //increments the number in Numberinator
        numberinator.display(); //displays the updated number and name
    }
}
