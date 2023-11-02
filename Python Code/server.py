import requests

# Specify the URL of the webpage you want to download
# Replace with the URL of the webpage you want to download
url = "https://www.amazon.in"

try:
    # Send an HTTP GET request to the URL
    response = requests.get(url)

    # Check if the request was successful (status code 200)
    if response.status_code == 200:
        # Get the HTML content from the response
        html_content = response.text

        # Save the HTML content to a local file
        with open("downloaded_page"+url+".html", "w", encoding="utf-8") as file:
            file.write(html_content)

        print("HTML content downloaded and saved to 'downloaded_page.html'")
    else:
        print(
            f"Failed to retrieve the webpage. Status code: {response.status_code}")
except requests.exceptions.RequestException as e:
    print(f"An error occurred: {e}")
