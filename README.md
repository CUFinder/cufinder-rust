# CUFinder Rust SDK

A Rust SDK for the CUFinder API that provides access to all company and person enrichment services.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cufinder-rust = "0.0.1"
```

## Usage

### Basic Usage

```rust
use cufinder_rust::CufinderSDK;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the SDK
    let sdk = CufinderSDK::new("your-api-key".to_string())?;
    
    // Get company domain
    let result = sdk.cuf("TechCorp", "US").await?;
    
    println!("Domain: {}", result.domain);
    Ok(())
}
```

### Advanced Configuration

```rust
use cufinder_rust::{CufinderSDK, ClientConfig};
use std::time::Duration;

let sdk = CufinderSDK::with_config(ClientConfig {
    api_key: "your-api-key".to_string(),
    base_url: "https://api.cufinder.io/v2".to_string(),
    timeout: Duration::from_secs(30),
    max_retries: 3,
})?;
```

## Services

### Company Services

#### CUF - Company URL Finder
```rust
let result = sdk.cuf("TechCorp", "US").await?;
// Returns: domain, query, credit_count
```

#### LCUF - LinkedIn Company URL Finder
```rust
let result = sdk.lcuf("TechCorp").await?;
// Returns: linkedin_url, query, credit_count
```

#### DTC - Domain to Company
```rust
let result = sdk.dtc("techcorp.com").await?;
// Returns: company_name, query, credit_count
```

#### DTE - Domain to Emails
```rust
let result = sdk.dte("techcorp.com").await?;
// Returns: emails[], query, credit_count
```

#### NTP - Name to Phones
```rust
let result = sdk.ntp("TechCorp").await?;
// Returns: phones[], query, credit_count
```

### Person Services

#### EPP - Enrich Profile
```rust
let result = sdk.epp("https://linkedin.com/in/john-doe").await?;
// Returns: person, company, query, credit_count
```

#### REL - Reverse Email Lookup
```rust
let result = sdk.rel("john.doe@techcorp.com").await?;
// Returns: person, company, query, credit_count
```

#### FWE - Find Work Email
```rust
let result = sdk.fwe("https://linkedin.com/in/john-doe").await?;
// Returns: email, query, credit_count
```

#### TEP - Person Enrichment
```rust
let result = sdk.tep("John Doe", "TechCorp").await?;
// Returns: person, query, confidence_level, credit_count
```

### Company Intelligence Services

#### FCL - Find Company Lookalikes
```rust
let result = sdk.fcl("TechCorp").await?;
// Returns: lookalikes[], query, credit_count
```

#### ELF - Enrich LinkedIn Fundraising
```rust
let result = sdk.elf("TechCorp").await?;
// Returns: fundraising, query, credit_count
```

#### CAR - Company Annual Revenue
```rust
let result = sdk.car("TechCorp").await?;
// Returns: revenue, query, credit_count
```

#### FCC - Find Company Children
```rust
let result = sdk.fcc("TechCorp").await?;
// Returns: subsidiaries[], query, credit_count
```

#### FTS - Find Tech Stack
```rust
let result = sdk.fts("TechCorp").await?;
// Returns: tech_stack, query, credit_count
```

#### ENC - Company Enrichment
```rust
let result = sdk.enc("TechCorp").await?;
// Returns: company, query, credit_count
```

#### CEC - Company Employee Countries
```rust
let result = sdk.cec("TechCorp").await?;
// Returns: countries[], total_results, query, credit_count
```

#### CLO - Company Locations
```rust
let result = sdk.clo("TechCorp").await?;
// Returns: locations[], query, credit_count
```

### Search Services

#### CSE - Company Search
```rust
use cufinder_rust::CseParams;

let result = sdk.cse(CseParams {
    name: Some("technology".to_string()),
    country: Some("US".to_string()),
    industry: Some("Technology".to_string()),
    page: Some(1),
    ..Default::default()
}).await?;
// Returns: companies[], total_results, page, query, credit_count
```

#### PSE - Person Search
```rust
use cufinder_rust::PseParams;

let result = sdk.pse(PseParams {
    full_name: Some("engineer".to_string()),
    company_name: Some("TechCorp".to_string()),
    page: Some(1),
    ..Default::default()
}).await?;
// Returns: people[], total_results, page, query, credit_count
```

#### LBS - Local Business Search
```rust
use cufinder_rust::LbsParams;

let result = sdk.lbs(LbsParams {
    name: Some("coffee".to_string()),
    city: Some("San Francisco".to_string()),
    page: Some(1),
    ..Default::default()
}).await?;
// Returns: businesses[], total_results, page, query, credit_count
```

## Error Handling

The SDK returns `Result<T, CufinderError>` for all operations:

```rust
match sdk.cuf("TechCorp", "US").await {
    Ok(result) => println!("Domain: {}", result.domain),
    Err(CufinderError::ValidationError(msg)) => eprintln!("Validation error: {}", msg),
    Err(CufinderError::AuthenticationError(msg)) => eprintln!("Auth error: {}", msg),
    Err(CufinderError::ApiError { status, message }) => eprintln!("API error {}: {}", status, message),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Testing

Run the tests:

```bash
cargo test
```

## Features

- **Async/Await Support**: Built with Tokio for async operations
- **Type Safety**: Strong typing with Serde for JSON serialization/deserialization
- **Error Handling**: Comprehensive error types with thiserror
- **No OpenSSL**: Uses rustls for TLS to avoid OpenSSL dependencies
- **Mock Testing**: Includes comprehensive test suite with mockito

## License

This project is licensed under the MIT License - see the LICENSE file for details.
