import os
import requests
from dotenv import load_dotenv

def interact_with_ai_api(user_message):
    # Load environment variables from .env file
    load_dotenv()

    # Retrieve the API endpoint and API key from environment variables
    api_endpoint = os.getenv("API_ENDPOINT")
    api_key = os.getenv("API_KEY")

    # Ensure the variables are set
    if not api_endpoint or not api_key:
        raise ValueError("API_ENDPOINT or API_KEY is missing in the .env file.")

    # Headers for the request
    headers = {
        "Content-Type": "application/json",
        "api-key": api_key,  # Assuming your API setup uses this header
        # "Authorization": f"Bearer {api_key}",  # Use this if Authorization Bearer is needed
    }

    # Payload for the request
    payload = {
        "messages": [
            {"role": "user", "content": user_message}
        ],
        "temperature": 0.7,
        "top_p": 0.95,
        "max_tokens": 800
    }

    # Send request
    try:
        response = requests.post(api_endpoint, headers=headers, json=payload)
        response.raise_for_status()  # Raises an HTTPError for bad responses (4xx or 5xx)
        return response.json()
    except requests.RequestException as e:
        raise SystemExit(f"Failed to make the request. Error: {e}")

if __name__ == "__main__":
    user_input = input("Enter your message to the AI: ")
    result = interact_with_ai_api(user_input)
    print("AI Response:")
    print(result)
