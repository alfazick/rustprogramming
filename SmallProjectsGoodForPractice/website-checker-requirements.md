# Website Status Checker Project Requirements

## Project Overview
Create a concurrent website monitoring system that can check the status of multiple websites simultaneously.

## Core Requirements

### Functionality
1. Must accept a list of website URLs for monitoring
2. Must support concurrent checking of websites
3. Must implement configurable timeout for each request
4. Must collect and report:
   - HTTP status code
   - Response time
   - Any errors encountered

### Performance Requirements
1. Must support checking at least 50 websites concurrently
2. Each request must timeout after a configurable duration (default: 5 seconds)
3. Must handle network failures gracefully

### Technical Requirements
1. Implementation must use Rust's threading system
2. Must use channels for communication between threads
3. Must implement proper error handling
4. Must support graceful shutdown

### Configuration Options
1. Number of worker threads
2. Request timeout duration
3. Maximum retries per website

### Output Format
```rust
struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time: Duration,
    timestamp: DateTime<Utc>,
}
```

## Bonus Features
1. Support for periodic monitoring
2. HTTP header validation
3. SSL certificate checking
4. Response body validation
5. Statistics collection (avg response time, uptime percentage)

## Testing Requirements
1. Unit tests for core functionality
2. Integration tests with mock HTTP server
3. Performance tests with multiple concurrent requests
4. Error handling tests
