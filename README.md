# URL Checker

A simple and fast asynchronous tool written in Rust to check the status of a list of URLs. It concurrently sends requests, measures response time, gets the HTTP status, and saves the results to a CSV file for analysis.

## How to Run

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/dorofi/url-checker.git
    cd url-checker
    ```

2.  **Create `urls.txt` file:**
    Create a file named `urls.txt` in the project's root directory and add a list of URLs, one per line.

    **Example `urls.txt`:**
    ```
    https://google.com
    https://github.com
    https://example.com/non-existent-page
    ```

3.  **Run the checker:**
    The easiest way is to use Cargo. This command will check the URLs from `urls.txt` and save the results to `report.csv`.
    ```bash
    cargo run --release
    ```

    **Example Output:**
    ```
    https://google.com 200 (56 ms)
    https://github.com 200 (123 ms)
    https://example.com/non-existent-page 404 (89 ms)
    ```
