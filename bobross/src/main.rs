use reqwest;

fn main() {
    let url = "https://www.bobrossquotes.com/text.php";

    // Create a blocking client
    let client = reqwest::blocking::Client::new();

    // Fetch the quote from the given URL
    let resp = match client.get(url).send() {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Failed to send request: {}", e);
            return;
        }
    };

    // Ensure we receive a successful response
    if !resp.status().is_success() {
        eprintln!("Error fetching the quote: {}", resp.status());
        return;
    }

    // Extract the text from the response
    let text = match resp.text() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to read response text: {}", e);
            return;
        }
    };

    // Print the formatted text to stdout
    println!("Bob Ross Quote:\n{}", text);
}
