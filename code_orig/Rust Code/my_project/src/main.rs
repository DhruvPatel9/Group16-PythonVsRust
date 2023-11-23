use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write, BufRead};
use reqwest::blocking::get;
use csv::Writer;

#[derive(Clone)]
struct Quote {
    text: String,
    author: String,
    favorite: bool,
}

fn download_html(url: &str) -> Result<String, Box<dyn Error>> {
    let response = get(url)?;
    if response.status().is_success() {
        let html_content = response.text()?;
        // Save HTML content to temp.html
        File::create("temp.html")?.write_all(html_content.as_bytes())?;
        Ok(html_content)
    } else {
        Err(format!("Failed to download HTML. Status code: {}", response.status()))?
    }
}

fn scrape_data(html: &str) -> Vec<Quote> {
    // Simplified regex patterns
    let quote_regex = Regex::new(r#"<span class="text" itemprop="text">(.+?)</span>"#).unwrap();
    let author_regex = Regex::new(r#"<small class="author" itemprop="author">(.+?)</small>"#).unwrap();

    // Use regular expressions to extract data
    let mut quotes = Vec::new();
    let mut i=1;
    for (quote_match, author_match) in quote_regex.captures_iter(html).zip(author_regex.captures_iter(html)) {
        // Print the entire match for debugging
        //println!("Quote Match: {:?}", quote_match.get(0).map(|m| m.as_str()));
        //println!("Author Match: {:?}", author_match.get(0).map(|m| m.as_str()));

        let text = quote_match.get(1).map_or("", |m| m.as_str()).trim().to_string();
        let author = author_match.get(1).map_or("", |m| m.as_str()).trim().to_string();

        // Print extracted quote and author for debugging
        println!("{}. Quote: {:?}, Author: {:?}", i , text, author);
        i=i+1;
        quotes.push(Quote {
            text: text.clone(),
            author: author.clone(),
            favorite: false,
        });
    }
    quotes
}




fn search_quotes_by_author<'a>(quotes: &'a [Quote], author: &str) -> Vec<&'a Quote> {
    quotes.iter().filter(|quote| quote.author == author).collect()
}

fn mark_as_favorite(quotes: &mut [Quote], quote_index: usize) -> Result<(), Box<dyn Error>> {
    if let Some(quote) = quotes.get_mut(quote_index) {
        quote.favorite = true;
        println!("Quote marked as favorite.");
        // Save favorite quotes to CSV file
        let favorite_quotes: Vec<Quote> = quotes.iter().filter(|quote| quote.favorite).cloned().collect();
        save_to_csv(favorite_quotes, "favorites.csv")?;
        println!("Favorite quotes saved to favorites.csv");
        Ok(())
    } else {
        Err("Invalid quote index".into())
    }
}

fn save_to_csv(quotes: Vec<Quote>, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_path(filename)?;
    writer.write_record(&["Quote", "Author", "Favorite"])?;
    for quote in quotes {
        writer.write_record(&[quote.text, quote.author, quote.favorite.to_string()])?;
    }
    writer.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Download HTML from the website
    let url = "https://quotes.toscrape.com/";
    let html = download_html(url)?;
    println!("HTML downloaded and saved to temp.html");

    // Scrape data from the HTML
    let mut quotes = scrape_data(&html);
    println!("Data scraped and stored in memory");

    // Save quotes to CSV file
    save_to_csv(quotes.clone(), "quotes.csv")?;
    println!("Quotes saved to quotes.csv");

    // Ask the user for input
    loop {
        println!("What do you want to do with the scraped data?");
        println!("1. Search quotes by author");
        println!("2. Mark a quote as favorite");
        println!("3. View favorite quotes");
        println!("4. Exit the program");
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input)?;
        let option = input.trim().parse::<u32>()?;
        match option {
            1 => {
                println!("Enter the name of an author to see their quotes:");
                let mut input = String::new();
                io::stdin().lock().read_line(&mut input)?;
                let author = input.trim();

                // Search quotes by author
                let author_quotes = search_quotes_by_author(&quotes, author);
                if author_quotes.is_empty() {
                    println!("No quotes found by {}", author);
                } else {
                    println!("Quotes by {}:", author);
                    for (i, quote) in author_quotes.iter().enumerate() {
                        println!("{}. {}", i + 1, quote.text);
                    }
                }
            }
            2 => {
                println!("Enter the index of a quote to mark it as favorite:");
                let mut input = String::new();
                io::stdin().lock().read_line(&mut input)?;
                let quote_index = input.trim().parse::<usize>()?;

                // Mark a quote as favorite
                mark_as_favorite(&mut quotes, quote_index - 1)?;
            }
            3 => {
                // View favorite quotes
                let favorite_quotes: Vec<&Quote> = quotes.iter().filter(|quote| quote.favorite).collect();
                if favorite_quotes.is_empty() {
                    println!("You have no favorite quotes.");
                } else {
                    println!("Your favorite quotes are:");
                    for (i, quote) in favorite_quotes.iter().enumerate() {
                        println!("{}. {} - {}", i + 1, quote.text, quote.author);
                    }
                }
            }
            4 => {
                // Exit the program
                println!("Thank you for using the web scraper. Goodbye!");
                break;
            }
            _ => {
                // Invalid option
                println!("Invalid option. Please enter a number between 1 and 4.");
            }
        }
    }

    Ok(())
}