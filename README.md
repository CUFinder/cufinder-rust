# CUFinder Rust SDK

[![](https://img.shields.io/badge/repo%20status-Active-28a745)](https://github.com/cufinder/cufinder-rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-514BEE.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/cufinder-rust.svg)](https://crates.io/crates/cufinder-rust)

A Rust SDK for the CUFinder API that provides access to all company and person enrichment services.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [API Reference](#api-reference)
- [Error Handling](#error-handling)
- [Types](#types)
- [Support](#support)

## Installation

```bash
cargo add cufinder-rust
```

## Usage

```rust
use cufinder_rust::CufinderSDK;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let sdk = CufinderSDK::new("your-api-key-here".to_string())?;
    
    // Initialize with more options
    let sdk = CufinderSDK::with_config(ClientConfig {
        api_key: "your-api-key-here".to_string(),
        base_url: "https://api.cufinder.io/v2".to_string(),
        timeout: Duration::from_secs(60),
        max_retries: 3,
    })?;
    
    Ok(())
}
```

## API Reference

This SDK covers all 28 Cufinder API (v2) endpoints:

- **CUF** - [Company Name to Domain](https://apidoc.cufinder.io/apis/company-name-to-domain)
- **LCUF** - [LinkedIn Company URL Finder](https://apidoc.cufinder.io/apis/company-linkedin-url-finder)
- **DTC** - [Domain to Company Name](https://apidoc.cufinder.io/apis/domain-to-company-name)
- **DTE** - [Company Email Finder](https://apidoc.cufinder.io/apis/company-email-finder)
- **NTP** - [Company Phone Finder](https://apidoc.cufinder.io/apis/company-phone-finder)
- **REL** - [Reverse Email Lookup](https://apidoc.cufinder.io/apis/reverse-email-lookup)
- **FCL** - [Company Lookalikes Finder](https://apidoc.cufinder.io/apis/company-lookalikes-finder)
- **ELF** - [Company Fundraising](https://apidoc.cufinder.io/apis/company-fundraising)
- **CAR** - [Company Revenue Finder](https://apidoc.cufinder.io/apis/company-revenue-finder)
- **FCC** - [Company Subsidiaries Finder](https://apidoc.cufinder.io/apis/company-subsidiaries-finder)
- **FTS** - [Company Tech Stack Finder](https://apidoc.cufinder.io/apis/company-tech-stack-finder)
- **EPP** - [LinkedIn Profile Enrichment](https://apidoc.cufinder.io/apis/linkedin-profile-enrichment)
- **FWE** - [LinkedIn Profile Email Finder](https://apidoc.cufinder.io/apis/linkedin-profile-email-finder)
- **TEP** - [Person Enrichment](https://apidoc.cufinder.io/apis/person-enrichment)
- **ENC** - [Company Enrichment](https://apidoc.cufinder.io/apis/company-enrichment)
- **CEC** - [Company Employee Count](https://apidoc.cufinder.io/apis/company-employee-count)
- **CLO** - [Company Locations](https://apidoc.cufinder.io/apis/company-locations)
- **CSE** - [Company Search](https://apidoc.cufinder.io/apis/company-search)
- **PSE** - [Person Search](https://apidoc.cufinder.io/apis/person-search)
- **LBS** - [Local Business Search (Google Maps Search API)](https://apidoc.cufinder.io/apis/local-business-search-google-maps-search-api)
- **BCD** - [B2B Customers Finder](https://apidoc.cufinder.io/apis/b2b-customers-finder)
- **CCP** - [Company Career Page Finder](https://apidoc.cufinder.io/apis/company-career-page-finder)
- **ISC** - [Company Saas Checker](https://apidoc.cufinder.io/apis/company-saas-checker)
- **CBC** - [Company B2B or B2C Checker](https://apidoc.cufinder.io/apis/company-b2b-or-b2c-checker)
- **CSC** - [Company Mission Statement](https://apidoc.cufinder.io/apis/company-mission-statement)
- **CSN** - [Company Snapshot](https://apidoc.cufinder.io/apis/company-snapshot)
- **NAO** - [Phone Number Normalizer](https://apidoc.cufinder.io/apis/phone-number-normalizer)
- **NAA** - [Address Normalizer](https://apidoc.cufinder.io/apis/address-normalizer)


**CUF - Company Name to Domain**

Returns the official website URL of a company based on its name.

```rust
let result = sdk.cuf("cufinder", "US").await?;
println!("{:?}", result);
```

**LCUF - LinkedIn Company URL Finder**

Finds the official LinkedIn company profile URL from a company name.

```rust
let result = sdk.lcuf("cufinder").await?;
println!("{:?}", result);
```

**DTC - Domain to Company Name**

Retrieves the registered company name associated with a given website domain.

```rust
let result = sdk.dtc("cufinder.io").await?;
println!("{:?}", result);
```

**DTE - Company Email Finder**

Returns up to five general or role-based business email addresses for a company.

```rust
let result = sdk.dte("cufinder.io").await?;
println!("{:?}", result);
```

**NTP - Company Phone Finder**

Returns up to two verified phone numbers for a company.

```rust
let result = sdk.ntp("apple").await?;
println!("{:?}", result);
```

**REL - Reverse Email Lookup**

Enriches an email address with detailed person and company information.

```rust
let result = sdk.rel("iain.mckenzie@stripe.com").await?;
println!("{:?}", result);
```

**FCL - Company Lookalikes Finder**

Provides a list of similar companies based on an input company's profile.

```rust
let result = sdk.fcl("apple").await?;
println!("{:?}", result);
```

**ELF - Company Fundraising**

Returns detailed funding information about a company.

```rust
let result = sdk.elf("cufinder").await?;
println!("{:?}", result);
```

**CAR - Company Revenue Finder**

Estimates a company's annual revenue based on name.

```rust
let result = sdk.car("apple").await?;
println!("{:?}", result);
```

**FCC - Company Subsidiaries Finder**

Identifies known subsidiaries of a parent company.

```rust
let result = sdk.fcc("amazon").await?;
println!("{:?}", result);
```

**FTS - Company Tech Stack Finder**

Detects the technologies a company uses.

```rust
let result = sdk.fts("cufinder").await?;
println!("{:?}", result);
```

**EPP - LinkedIn Profile Enrichment**

Takes a LinkedIn profile URL and returns enriched person and company data.

```rust
let result = sdk.epp("linkedin.com/in/iain-mckenzie").await?;
println!("{:?}", result);
```

**FWE - LinkedIn Profile Email Finder**

Extracts a verified business email address from a LinkedIn profile URL.

```rust
let result = sdk.fwe("linkedin.com/in/iain-mckenzie").await?;
println!("{:?}", result);
```

**TEP - Person Enrichment**

Returns enriched person data based on full name and company name.

```rust
let result = sdk.tep("iain mckenzie", "stripe").await?;
println!("{:?}", result);
```

**ENC - Company Enrichment**

Provides a complete company profile from a company name.

```rust
let result = sdk.enc("cufinder").await?;
println!("{:?}", result);
```

**CEC - Company Employee Count**

Returns an estimated number of employees for a company.

```rust
let result = sdk.cec("cufinder").await?;
println!("{:?}", result);
```

**CLO - Company Locations**

Returns the known physical office locations of a company.

```rust
let result = sdk.clo("apple").await?;
println!("{:?}", result);
```

**CSE - Company Search**

Search for companies by keyword, partial name, industry, location, or other filters.

```rust
use cufinder_rust::CseParams;

let result = sdk.cse(CseParams {
    name: Some("cufinder".to_string()),
    country: Some("germany".to_string()),
    state: Some("hamburg".to_string()),
    city: Some("hamburg".to_string()),
    ..Default::default()
}).await?;
println!("{:?}", result);
```

**PSE - Person Search**

Search for people by name, company, job title, location, or other filters.

```rust
use cufinder_rust::PseParams;

let result = sdk.pse(PseParams {
    full_name: Some("iain mckenzie".to_string()),
    company_name: Some("stripe".to_string()),
    ..Default::default()
}).await?;
println!("{:?}", result);
```

**LBS - Local Business Search (Google Maps Search API)**

Search for local businesses by location, industry, or name.

```rust
use cufinder_rust::LbsParams;

let result = sdk.lbs(LbsParams {
    country: Some("united states".to_string()),
    state: Some("california".to_string()),
    page: Some(1),
    ..Default::default()
}).await?;
println!("{:?}", result);
```

**BCD - B2B Customers Finder**

Returns company's careers page

```rust
let result = sdk.bcd("stripe.com").await?;
println!("{:?}", result);
```

**CCP - Company Career Page Finder**

Returns is company SaaS or not

```rust
let result = sdk.ccp("stripe.com").await?;
println!("{:?}", result);
```

**ISC - Company Saas Checker**

Returns is company SaaS or not

```rust
let result = sdk.isc("stripe.com").await?;
println!("{:?}", result);
```

**CBC - Company B2B or B2C Checker**

Returns company's business type

```rust
let result = sdk.cbc("stripe.com").await?;
println!("{:?}", result);
```

**CSC - Company Mission Statement**

Returns company's mission statement

```rust
let result = sdk.csc("stripe.com").await?;
println!("{:?}", result);
```

**CSN - Company Snapshot**

Returns company's snapshot information

```rust
let result = sdk.csn("stripe.com").await?;
println!("{:?}", result);
```

**NAO - Phone Number Normalizer**

Returns normalized phone

```rust
let result = sdk.nao("+18006676389").await?;
println!("{:?}", result);
```

**NAA - Address Normalizer**

Returns normalized address

```rust
let result = sdk.naa("1095 avenue of the Americas, 6th Avenue ny 10036").await?;
println!("{:?}", result);
```

## Error Handling

The SDK provides comprehensive error handling with custom error types:

```rust
use cufinder_rust::{CufinderSDK, CufinderError};

match sdk.cuf("cufinder", "US").await {
    Ok(result) => {
        println!("Domain: {}", result.domain);
    }
    Err(CufinderError::AuthenticationError(msg)) => {
        // 401 - Invalid API key
        eprintln!("Authentication failed: {}", msg);
    }
    Err(CufinderError::CreditLimitError(msg)) => {
        // 400 - Not enough credit
        eprintln!("Not enough credit: {}", msg);
    }
    Err(CufinderError::NotFoundError(msg)) => {
        // 404 - Not found result
        eprintln!("Not found result: {}", msg);
    }
    Err(CufinderError::PayloadError(msg)) => {
        // 422 - Error in the payload
        eprintln!("Payload error: {}", msg);
    }
    Err(CufinderError::RateLimitError(msg)) => {
        // 429 - Rate limit exceeded
        eprintln!("Rate limit exceeded: {}", msg);
    }
    Err(CufinderError::ServerError { status, message }) => {
        // 500, 501, ... - Server errors
        eprintln!("Server error ({}): {}", status, message);
    }
    Err(CufinderError::NetworkError(msg)) => {
        eprintln!("Network error: {}", msg);
    }
    Err(CufinderError::ValidationError(msg)) => {
        eprintln!("Validation error: {}", msg);
    }
    Err(e) => {
        eprintln!("Unknown error: {}", e);
    }
}
```

## Types

The SDK exports comprehensive Rust types for all API requests and responses:

```rust
// Request parameter types
#[derive(Debug, Clone, Default, Serialize)]
pub struct CseParams {
    pub name: Option<String>,
    pub domain: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub industry: Option<String>,
    pub company_size: Option<String>,
    pub revenue: Option<String>,
    pub employee_count: Option<String>,
    pub page: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PseParams {
    pub full_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub company_domain: Option<String>,
    pub job_title: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub page: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LbsParams {
    pub name: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub industry: Option<String>,
    pub page: Option<i32>,
}

// Response types
#[derive(Debug, Clone, Deserialize)]
pub struct BaseResponse {
    pub query: String,
    pub credit_count: i32,
}

// Model types
#[derive(Debug, Clone, Deserialize)]
pub struct Company {
    // The Company struct contains all returned company data.
    pub name: Option<String>,
    pub domain: Option<String>,
    pub website: Option<String>,
    pub linkedin_url: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
    pub industry: Option<String>,
    pub company_size: Option<String>,
    pub revenue: Option<String>,
    pub employee_count: Option<i32>,
    pub subsidiaries: Option<Vec<String>>,
    pub tech_stack: Option<Vec<String>>,
    pub emails: Option<Vec<String>>,
    pub phones: Option<Vec<String>>,
    pub description: Option<String>,
    pub locations: Option<Vec<CompanyLocation>>,
    pub founded_year: Option<i32>,
    pub logo_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Person {
    // The Person struct contains all returned person data.
    pub full_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub company_domain: Option<String>,
    pub job_title: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub description: Option<String>,
    pub linkedin_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LookalikeCompany {
    // The LookalikeCompany struct contains all returned lookalike company data.
    pub name: Option<String>,
    pub domain: Option<String>,
    pub website: Option<String>,
    pub linkedin_url: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
    pub industry: Option<String>,
    pub company_size: Option<String>,
    pub revenue: Option<String>,
    pub employee_count: Option<i32>,
    pub subsidiaries: Option<Vec<String>>,
    pub tech_stack: Option<Vec<String>>,
    pub emails: Option<Vec<String>>,
    pub phones: Option<Vec<String>>,
    pub description: Option<String>,
    pub locations: Option<Vec<CompanyLocation>>,
    pub founded_year: Option<i32>,
    pub logo_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FundraisingInfo {
    pub funding_last_round_type: Option<String>,
    pub funding_ammount_currency_code: Option<String>,
    pub funding_money_raised: Option<String>,
    pub funding_last_round_investors_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompanyLocation {
    // The CompanyLocation struct contains all returned company location data.
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}

// Configuration
pub struct ClientConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout: Duration,
    pub max_retries: u32,
}

// Error types
pub enum CufinderError {
    AuthenticationError(String),
    CreditLimitError(String),
    NotFoundError(String),
    PayloadError(String),
    RateLimitError(String),
    ServerError { status: u16, message: String },
    NetworkError(String),
    ValidationError(String),
}
```

## Support

For support, please open an issue on the [GitHub repository](https://github.com/cufinder/cufinder-rust/issues).
