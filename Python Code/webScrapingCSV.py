import requests
from bs4 import BeautifulSoup
import csv

url = "http://quotes.toscrape.com"
response = requests.get(url)

if response.status_code == 200:
    soup = BeautifulSoup(response.text, "lxml")
    
    quotes = soup.find_all("span", class_="text")
    authors = soup.find_all("small", class_="author")
    
    # Create a list to store the data
    data = []
    
    for quote, author in zip(quotes, authors):
        data.append([quote.text, author.text])

    # Specify the CSV file name
    csv_file = "quotes.csv"
    
    # Write the data to the CSV file
    with open(csv_file, mode="w", newline="") as file:
        writer = csv.writer(file)
        
        # Write the header row
        writer.writerow(["Quote", "Author"])
        
        # Write the data rows
        writer.writerows(data)
    
    print(f"Data has been saved to {csv_file}")
else:
    print(f"Failed to retrieve the webpage. Status code: {response.status_code}")
