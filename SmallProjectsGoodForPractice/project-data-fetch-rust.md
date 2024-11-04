# Financial Data Fetcher in Rust

## Overview
This project aims to build a Rust application that periodically fetches and records the pricing data of Bitcoin, Ethereum, and the S&P 500 index. The application will:
* Define three structs: `Bitcoin`, `Ethereum`, and `SP500`.
* Create a `Pricing` trait and implement it for each struct.
* Utilize the `ureq` library for HTTP requests and the `serde` library for JSON parsing.
* Continuously fetch data every 10 seconds.
* Save the fetched data into three separate files.
* Provide a screenshot of the running program.
* Share a GitHub repository link containing the source code.



### Learning Resources
* **Rust Traits Guide**: [GitHub - Rust Traits Example](https://github.com/alfazick/rustprogramming/blob/main/Module6Generics%2BTrait/02rust-traits-guide.md)
* **Error Handling Example**: [GitHub - Rust Enums and Error Handling](https://github.com/alfazick/rustprogramming/blob/main/Module5Error_Handling/02rust-enums-dog-api-example.md)

## Objectives
* **Struct Creation**: Define data structures for Bitcoin, Ethereum, and SP500.
* **Trait Implementation**: Standardize the data fetching mechanism using a `Pricing` trait.
* **HTTP Requests**: Use `ureq` to interact with public APIs for real-time pricing.
* **Data Parsing**: Deserialize JSON responses into Rust structs with `serde`.
* **Data Storage**: Write pricing data to individual files for each asset.
* **Periodic Execution**: Implement a loop to fetch and save data every 10 seconds.
* **Documentation**: Comment the code for clarity and maintain a README file.
* **Demonstration**: Provide a screenshot and GitHub link for validation.

## Components

### 1. Structs
* **Bitcoin**: Represents Bitcoin pricing data.
* **Ethereum**: Represents Ethereum pricing data.
* **SP500**: Represents S&P 500 index data.

### 2. Pricing Trait
Define a `Pricing` trait with the following methods:
* `fetch_price()`: Fetches the latest price from an API.
* `save_to_file()`: Saves the fetched price to a file.

### 3. Libraries
* **ureq**: For making HTTP GET requests.
* **serde**: For parsing JSON data into Rust structs.

### 4. Main Function
* Create a vector containing instances of `Bitcoin`, `Ethereum`, and `SP500`.
* Implement a loop that:
   * Calls `fetch_price()` for each asset.
   * Calls `save_to_file()` to record the data.
   * Waits for 10 seconds before repeating.

## Execution Flow
1. **Initialization**: Instantiate the structs and store them in a vector.
2. **Data Fetching Loop**:
   * Iterate over each asset.
   * Fetch and save the latest pricing data.
   * Pause for 10 seconds.
3. **Termination**: The program runs indefinitely until manually stopped.

## Deliverables
* **Source Code**: Available on a GitHub repository with clear commit history.
* **Screenshot**: Showing the terminal output of the running program.

## GitHub Repository
Please provide the link to your GitHub repository containing the source code once the project is completed.

## Additional Notes
* Implement proper error handling for network requests and file operations.
* Ensure the data in the files is well-formatted for future analysis.
* Study the provided learning resources for understanding traits and error handling implementation.

By following this project description, you'll create a Rust application that not only fetches and records financial data but also demonstrates the use of traits, structs, and external libraries in Rust.
