# Complete Rust API Client for Language Model Interaction

Here's the full, untruncated code for the Rust API client:

```rust
use dotenv::dotenv;
use std::env;
use serde::{Deserialize, Serialize};
use ureq;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct RequestPayload {
    messages: Vec<Message>,
    temperature: f32,
    top_p: f32,
    max_tokens: u32,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
    finish_reason: String,
    index: u32,
}

#[derive(Deserialize, Debug)]
struct ResponsePayload {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Deserialize, Debug)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Retrieve the API endpoint and API key from environment variables
    let api_endpoint = env::var("API_ENDPOINT")
        .expect("API_ENDPOINT not set in .env file");
    let api_key = env::var("API_KEY")
        .expect("API_KEY not set in .env file");

    // Construct the request payload
    let payload = RequestPayload {
        messages: vec![
            Message {
                role: "user".to_string(),
                content: "Tell me a fun fact about space.".to_string(),
            },
        ],
        temperature: 0.7,
        top_p: 0.95,
        max_tokens: 800,
    };

    // Send the POST request using ureq
    let response = ureq::post(&api_endpoint)
        .set("Content-Type", "application/json")
        .set("api-key", &api_key)
        .send_json(&payload)?;

    // Print raw response for debugging
    println!("Raw response: {}", response.into_string()?);

    // Send the request again to parse the JSON
    let response = ureq::post(&api_endpoint)
        .set("Content-Type", "application/json")
        .set("api-key", &api_key)
        .send_json(&payload)?;

    // Handle the response
    match response.into_json::<ResponsePayload>() {
        Ok(response_payload) => {
            if let Some(choice) = response_payload.choices.first() {
                println!("Generated text: {}", choice.message.content);
            } else {
                println!("No response generated.");
            }
            println!("Full response: {:?}", response_payload);
        }
        Err(e) => {
            eprintln!("Failed to parse response: {}", e);
        }
    }

    Ok(())
}
```

## Explanation

This code includes all the necessary components for interacting with a language model API:

1. It uses `dotenv` to load environment variables from a `.env` file.
2. It defines structures for both the request (`RequestPayload`) and response (`ResponsePayload`, `Choice`, `Usage`).
3. It sends a POST request to the API with the specified payload.
4. It prints the raw response for debugging purposes.
5. It then sends another request and attempts to parse the JSON response.
6. If successful, it prints the generated text and the full response payload.
7. If there's an error in parsing, it prints an error message.

## Running the Application

To run this application:

1. Create a `.env` file in the project root with the following content:
   ```
   API_ENDPOINT=<your_api_endpoint_here>
   API_KEY=<your_api_key_here>
   ```

2. Ensure you have the necessary dependencies in your `Cargo.toml`:
   ```toml
   [dependencies]
   dotenv = "0.15.0"
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   ureq = { version = "2.5", features = ["json"] }
   ```

3. Run the application with:
   ```
   cargo run
   ```

This code should now provide a complete implementation of the API client, including error handling and debugging output. If you encounter any issues, the raw response output should help in identifying the problem.
