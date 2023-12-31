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

## Web scrapping: 
Web scraping is a data extraction technique employed to gather information from websites by simulating the behavior of a web browser. This process involves programmatically accessing and parsing the HTML structure of web pages, extracting specific data points or content, and transforming it into a structured format for analysis or storage. Web scraping is commonly used for various purposes, including gathering real-time data for research, monitoring prices on e-commerce sites, tracking changes in online content, and aggregating information from multiple sources. While it can provide valuable insights and automate data collection, web scraping must be conducted ethically and in compliance with legal and ethical standards. It is essential for practitioners to be mindful of website terms of service, adhere to robots.txt guidelines, and respect intellectual property rights, ensuring responsible and respectful use of the web as a data source.
 
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

## Output Comparison
### Rust is faster compared to Python, but takes up more memory
Analysis was done by creating a massif file and generating a report from it.<br>
We used massif since it is used for  detailed heap profiling and is also platform independant, hence can be shared and used by anybody to generate a report.<br>
### The following results were obtained for Rust:

![image](https://github.com/DhruvPatel9/Group16-PythonVsRust/assets/101914758/3dc759e3-cc72-4d61-adb6-474ce238cdaf)


### The following results were obtained for Python:

![image](https://github.com/DhruvPatel9/Group16-PythonVsRust/assets/101914758/f60584ae-ac6e-402b-96bc-45bc8968c853)

# Scope of Future Work
### Configurability:
Make certain parameters configurable, such as the URLs, CSV filenames, or other constants, allowing for easier adaptation to different websites or use cases.

### Handling Dynamic Content:
If the target websites use JavaScript or have dynamic content loading, consider using tools like Selenium for a more dynamic and comprehensive scraping approach.

### Refactoring for Modularity:
Consider refactoring the code for increased modularity. Breaking down the functionalities into smaller, focused functions or classes can enhance readability and maintainability.


### Caching Mechanism:
Implement a caching mechanism to store previously scraped data locally. This can reduce the need to re-scrape data on every run, improving efficiency and reducing the load on the web server.

### Automated Testing:
Introduce automated testing using frameworks such as pytest. This can help ensure the correctness of the code and facilitate future modifications without introducing regressions.

