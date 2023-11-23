# Group16-PythonVsRust
## Principle of Programming Language Project link of Group 16

## Project Members
1. Pranav Pawar - 2021A7PS2599G
2. Dhruv Patel - 2021A7PS2672G
3. Kislay Ranjan Nee Tandon - 2021A7PS2627G
4. Tejas Sovani - 2021A7PS2720G

## Problem Statement 
The objective of this project is to conduct a comparative analysis of Web Scraping implementations in Python and Rust, with a particular emphasis on evaluating their performance, memory safety, and adherence to functional programming principles. The project will involve designing and implementing web scraping algorithms in both languages, considering factors such as execution time, CPU utilization, and dynamic memory allocation. The primary focus will be on assessing the efficiency and reliability of each implementation in the context of processing web data, while also exploring how Rust's memory safety features and Python's functional programming capabilities contribute to the overall performance and robustness of the solutions.

## POPL Ascepts Involved
During the course of web scraping, this project delves into the POPL (Programming Languages Principles) aspects, the project aims to uncover insights into the interplay of functional programming paradigms, memory safety features, ownership models, and regular expression capabilities in both Python and Rust, while concurrently assessing their impact on performance during web data extraction.

## Software Architecture
### Components:
Web Scraper: Responsible for downloading HTML content, parsing it using BeautifulSoup, and extracting relevant data.
CSV Handler: Manages the writing of data to CSV files.
User Interface/Interaction: The while loop that continuously prompts the user for input and executes actions based on the user's choice.	

### Reuse and Development:
Reuse: The code leverages existing libraries such as requests for HTTP requests and BeautifulSoup for HTML parsing. These are reusable components developed by others.
Development: The application-specific logic, including the structure of the main loop, user interaction, and handling of scraped data, is developed for this specific application.

### Client-Server Architecture:
The provided code does not exhibit a client-server architecture. It's a standalone command-line application where the user interacts with the application locally.

### Testing Component:
Testing appears to be primarily manual. The code does not explicitly include automated testing components or frameworks. Testing is likely performed by running the application and interacting with it through the command line.

### Database:
The code does not involve a traditional database. Instead, it uses CSV files to persist scraped data and favorite quotes.

## Comparison of Python's ease of use for Web Scraping with Rust's safety and performance 

## Web scrapping: 
 Web scraping is the process of automating data extraction from websites, utilizing bots or scripts to navigate and retrieve information. It enables the collection of text, images, links, and more by sending HTTP requests and parsing HTML.
 
 ##  TASK done till Milestone 1:
We wrote Python code for downloading HTML Script and then in the same file, we added code for Web Scrapping it 
In python we used the 'request' and 'Beautiful Soup' library to implement Web Scraping. The purpose of 'requests' library is used for making HTTP requests. 'Beautiful Soup' library is used to 

 ## Principles of Programming Language Used:
 ### Ownership and Lifetimes:
    Lines 10-11: struct Quote { ... } demonstrates ownership semantics with the String type.<br>
    Lines 34, 36, 39, 42, and 44: Ownership of the text and author strings in the scrape_data function.

### Referencing and Borrowing:
    Lines 65, 77, 79, and 85: Borrowing references in function parameters.
    Line 103: Mutable reference (&mut) in the mark_as_favorite function.

### Mutability:
    Line 86: The quotes vector is mutable in the main function.
    Lines 100-104: Modification of the favorite field using a mutable 
    reference in the mark_as_favorite function.

### Pattern Matching:
    Lines 28-31: Pattern matching with regular expressions in the scrape_data 
    function.

### Traits:
    Line 7: #[derive(Clone)] implements the Clone trait for the Quote struct.
    Line 37: Using the Error trait for error handling.
## Advantages of each language:
### Rust:
* Memory Safety:
Rust's ownership system ensures memory safety, preventing common memory-related errors in the web scraping code.

* Performance:
Rust's design for high performance allows low-level control over system resources, enhancing efficiency in web scraping tasks.<br>
Rust's performance benefits are utilized in web scraping, as it's designed for low-level control without sacrificing safety.

* Concurrency:
Rust's ownership system prevents data races, making concurrent programming in web scraping safer and more manageable.

* Expressive Type System:
Rust's type system enables concise and clear implementations, enhancing the maintainability of web scraping code.

### Python:
* Ease of Implementation:
Python's readability and simplicity make it easy to implement web scraping algorithms, prioritizing ease of development.

* Rapid Prototyping:
Python's dynamic typing and high-level abstractions facilitate rapid prototyping for quick experimentation in web scraping.

* Community and Libraries:
Python's active community and rich library ecosystem simplify the implementation of web scraping algorithms, leveraging existing data structures and utilities.

* Interpreted Language:
Python's interpreted nature allows for quick testing and debugging, streamlining the iterative development process in web scraping.

	

