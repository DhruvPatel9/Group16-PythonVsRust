import requests
from bs4 import BeautifulSoup
import csv

def download_html(url):
    response = requests.get(url)
    if response.status_code == 200:
        return response.text
    else:
        print(f"Failed to download the HTML. Status code: {response.status_code}")
        return None

def scrape_data(html):
    soup = BeautifulSoup(html, 'html.parser')

    quotes = soup.find_all('span', class_='text')
    authors = soup.find_all('small', class_='author')

    data = []
    for quote, author in zip(quotes, authors):
        data.append({'quote': quote.get_text(strip=True), 'author': author.get_text(strip=True), 'favorite': False})

    return data

def save_to_csv(data, filename):
    with open(filename, 'w', newline='', encoding='utf-8') as csvfile:
        fieldnames = ['Quote', 'Author', 'Favorite']
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)

        writer.writeheader()
        for entry in data:
            writer.writerow({'Quote': entry['quote'], 'Author': entry['author'], 'Favorite': entry['favorite']})

def search_quotes_by_author(quotes, author):
    return list(filter(lambda quote: quote['author'] == author, quotes))

def mark_as_favorite(quotes, quote_index):
    if 0 <= quote_index < len(quotes):
        quotes[quote_index]['favorite'] = True
        print("Quote marked as favorite.")
    else:
        print("Invalid quote index.")

def save_favorites_to_csv(quotes, filename):
    favorites = [quote for quote in quotes if quote['favorite']]
    save_to_csv(favorites, filename)

def display_favorite_quotes(quotes):
    favorites = [quote['quote'] for quote in quotes if quote['favorite']]
    if favorites:
        print("\nFavorite Quotes:")
        for i, quote in enumerate(favorites):
            print(f"{i + 1}. {quote}")
    else:
        print("\nNo favorite quotes.")

if __name__ == "__main__":
    # URL of the webpage containing quotes
    url = "http://quotes.toscrape.com"

    # Download HTML
    html_content = download_html(url)

    if html_content:
        # Scrape data
        scraped_data = scrape_data(html_content)

        # Save to CSV
        save_to_csv(scraped_data, 'quotes.csv')

        while True:
            print("\nOptions:")
            print("1. Search for quotes by author")
            print("2. Mark a quote as favorite")
            print("3. Display favorite quotes")
            print("4. End program")

            option = input("Enter the option number: ")

            if option == '1':
                # Search for quotes by author
                author_to_search = input("Enter the author's name: ")
                results = search_quotes_by_author(scraped_data, author_to_search)

                if results:
                    print(f"Quotes by {author_to_search}:")
                    for i, quote in enumerate(results):
                        print(f"{i + 1}. \"{quote['quote']}\"")
                else:
                    print(f"No quotes found for {author_to_search}")

            elif option == '2':
                # Mark a quote as a favorite
                print("\nAll Quotes:")
                for i, quote in enumerate(scraped_data):
                    print(f"{i + 1}. \"{quote['quote']}\"")
                
                favorite_index = int(input("Enter the index of the quote to mark as favorite (0 to exit): "))
                while favorite_index != 0:
                    mark_as_favorite(scraped_data, favorite_index - 1)
                    favorite_index = int(input("Enter the index of the quote to mark as favorite (0 to exit): "))

                # Save favorites to CSV
                save_favorites_to_csv(scraped_data, 'favorites.csv')

            elif option == '3':
                # Display favorite quotes
                display_favorite_quotes(scraped_data)

            elif option == '4':
                print("Program ended.")
                break

            else:
                print("Invalid option. Please enter a valid option.")
