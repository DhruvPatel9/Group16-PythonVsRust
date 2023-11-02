from bs4 import BeautifulSoup



# Read the HTML content from the file
with open("htmlsample.html", "r", encoding="utf-8") as file:
    html_content = file.read()

# Create a BeautifulSoup object to parse the HTML
soup = BeautifulSoup(html_content, "html.parser")

# Extract and print the text from specific elements
event_title = soup.find("h1").text
event_description = soup.find("p", string="The fanciest cat event.").text
event_date = soup.find(
    "p", string="The 115th annual Cat Fancy Event is being held in New York City on December 21st.").text
featured_cats = [li.text for li in soup.select(
    "h2:contains('Featured Cats') + ul li")]
footer_text = soup.find("footer").find("p").text
# Print the extracted information
print("Event Title:", event_title)
print("Event Description:", event_description)
print("Event Date:", event_date)
print("Featured Cats:", featured_cats)
print("Footer Text:", footer_text)
