// Import the required libraries
extern crate reqwest;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use reqwest::blocking::get;

// Define the main function
fn main() {
    // Define the URL to download from
    let url = "https://www.flipkart.com/search?q=mobiles&as=on&as-show=on&otracker=AS_Query_TrendingAutoSuggest_1_0_na_na_na&otracker1=AS_Query_TrendingAutoSuggest_1_0_na_na_na&as-pos=1&as-type=TRENDING&suggestionId=mobiles&requestId=f53bfed8-6878-4a9a-9405-1bd406e2f3a6";
    // Define the file name to store the HTML code
    let file_name = "flipkart.html";
    // Call the download function with the URL and file name as arguments
    download(url, file_name).expect("Download failed");
}

// Define the download function that takes a URL and a file name as parameters
// and returns a Result type
fn download(url: &str, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create a new file with the given file name
    let mut file = File::create(file_name)?;
    // Send a GET request to the URL and get the response
    let response = get(url)?;
    // Create a buffered reader from the response
    let reader = BufReader::new(response);
    // Iterate over the lines in the reader
    for line in reader.lines() {
        // Write each line to the file with a newline character
        writeln!(file, "{}", line?)?;
    }
    // Return Ok if everything went well
    Ok(())
}
