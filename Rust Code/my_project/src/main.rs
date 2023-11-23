use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
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
    // Define regular expressions to match patterns in HTML
    let quote_regex = Regex::new(r#"<span class="text">(.*?)</span>"#).unwrap();
    let author_regex = Regex::new(r#"<small class="author">(.*?)</small>"#).unwrap();

    // Use regular expressions to extract data
    let mut quotes = Vec::new();
    for (quote_match, author_match) in quote_regex.captures_iter(html).zip(author_regex.captures_iter(html)) {
        let text = quote_match.get(1).map_or("", |m| m.as_str()).trim().to_string();
        let author = author_match.get(1).map_or("", |m| m.as_str()).trim().to_string();

        quotes.push(Quote {
            text,
            author,
            favorite: false,
        });
    }

    quotes
}





fn search_quotes_by_author<'a>(quotes: &'a [Quote], author: &str) -> Vec<&'a Quote> {
    quotes.iter().filter(|quote| quote.author == author).collect()
}

fn mark_as_favorite(quotes: &mut [Quote], quote_index: usize) {
    if let Some(quote) = quotes.get_mut(quote_index) {
        quote.favorite = true;
        println!("Quote marked as favorite.");
    } else {
        println!("Invalid quote index.");
    }
}

fn save_to_csv(quotes: Vec<Quote>, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_path(filename)?;

    writer.write_record(&["Quote", "Author", "Favorite"])?;

    for quote in quotes {
        writer.write_record(&[
            &quote.text,
            &quote.author,
            &quote.favorite.to_string(),
        ])?;
    }

    writer.flush()?; // Flush the writer to ensure all records are written
    writer.into_inner()?; // Close the writer to release the file handle
    Ok(())
}

fn save_favorites_to_csv(quotes: Vec<Quote>, filename: &str) -> Result<(), Box<dyn Error>> {
    let favorites: Vec<_> = quotes.iter().filter(|quote| quote.favorite).cloned().collect();
    save_to_csv(favorites, filename)
}







fn display_favorite_quotes(quotes: &[Quote]) {
    let favorites: Vec<_> = quotes.iter().filter(|quote| quote.favorite).collect();
    if !favorites.is_empty() {
        println!("Favorite Quotes:");
        favorites.iter().enumerate().for_each(|(i, quote)| {
            println!("{}. {}", i + 1, quote.text);
        });
    } else {
        println!("No favorite quotes.");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // URL of the webpage containing quotes
    let url = "http://quotes.toscrape.com";

    // Download HTML and save to temp.html
    let html_content = download_html(url)?;

    // Scrape data
    let mut scraped_data = scrape_data(&html_content);

    // Save to CSV (all quotes)
    save_to_csv(scraped_data.clone(), "quotes.csv")?;

    loop {
        println!("\nOptions:");
        println!("1. Search for quotes by author");
        println!("2. Mark a quote as favorite");
        println!("3. Display favorite quotes");
        println!("4. End program");

        let mut option = String::new();
        io::stdin().read_line(&mut option)?;
        let option = option.trim();

        match option {
            "1" => {
                // Search for quotes by author
                println!("Enter the author's name: ");
                let mut author_to_search = String::new();
                io::stdin().read_line(&mut author_to_search)?;
                let author_to_search = author_to_search.trim();

                let results = search_quotes_by_author(&scraped_data, author_to_search);
                if !results.is_empty() {
                    println!("Quotes by {}: ", author_to_search);
                    results.iter().enumerate().for_each(|(i, quote)| {
                        println!("{}. \"{}\"", i + 1, quote.text);
                    });
                } else {
                    println!("No quotes found for {}", author_to_search);
                }
            }
            "2" => {
                // Mark a quote as a favorite
                println!("All Quotes:");
                scraped_data.iter().enumerate().for_each(|(i, quote)| {
                    println!("{}. \"{}\"", i + 1, quote.text);
                });

                println!("Enter the index of the quote to mark as favorite (0 to exit): ");
                let mut favorite_index = String::new();
                io::stdin().read_line(&mut favorite_index)?;
                let favorite_index: usize = match favorite_index.trim().parse() {
                    Ok(index) => index,
                    Err(_) => continue,
                };

                if favorite_index == 0 {
                    break;
                }

                mark_as_favorite(&mut scraped_data, favorite_index - 1);

                // Save favorites to CSV
                save_favorites_to_csv(scraped_data.clone(), "favorites.csv")?;
            }
            "3" => {
                // Display favorite quotes
                display_favorite_quotes(&scraped_data);
            }
            "4" => {
                println!("Program ended.");
                break;
            }
            _ => {
                println!("Invalid option. Please enter a valid option.");
            }
        }
    }

    Ok::<(), Box<dyn Error>>(())

}
