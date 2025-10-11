use cufinder_rust::{
    CufinderSDK, ClientConfig, CseParams, PseParams, LbsParams,
    CufinderError,
};
use mockito::Server;
use serde_json::json;
use std::time::Duration;

#[tokio::test]
async fn test_cuf_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/cuf")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "domain": "techcorp.com",
            "query": "TechCorp",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.cuf("TechCorp", "US").await.unwrap();

    assert_eq!(result.domain, "techcorp.com");
    assert_eq!(result.base.query, Some("TechCorp".to_string()));
    assert_eq!(result.base.credit_count, Some(1));
}

#[tokio::test]
async fn test_lcuf_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/lcuf")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "linkedin_url": "https://linkedin.com/company/techcorp",
            "query": "TechCorp",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.lcuf("TechCorp").await.unwrap();

    assert_eq!(result.linkedin_url, "https://linkedin.com/company/techcorp");
    assert_eq!(result.base.query, Some("TechCorp".to_string()));
}

#[tokio::test]
async fn test_dtc_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/dtc")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "company_name": "TechCorp Inc",
            "query": "techcorp.com",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.dtc("techcorp.com").await.unwrap();

    assert_eq!(result.company_name, "TechCorp Inc");
    assert_eq!(result.base.query, Some("techcorp.com".to_string()));
}

#[tokio::test]
async fn test_dte_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/dte")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "emails": ["contact@techcorp.com", "info@techcorp.com"],
            "query": "techcorp.com",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.dte("techcorp.com").await.unwrap();

    assert_eq!(result.emails.len(), 2);
    assert!(result.emails.contains(&"contact@techcorp.com".to_string()));
    assert!(result.emails.contains(&"info@techcorp.com".to_string()));
}

#[tokio::test]
async fn test_ntp_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/ntp")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "phones": ["+1-555-0123", "+1-555-0124"],
            "query": "TechCorp",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.ntp("TechCorp").await.unwrap();

    assert_eq!(result.phones.len(), 2);
    assert!(result.phones.contains(&"+1-555-0123".to_string()));
    assert!(result.phones.contains(&"+1-555-0124".to_string()));
}

#[tokio::test]
async fn test_rel_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/rel")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "person": {
                "full_name": "John Doe",
                "email": "john.doe@techcorp.com",
                "job_title": "Software Engineer"
            },
            "company": {
                "name": "TechCorp",
                "domain": "techcorp.com",
                "industry": "Technology"
            },
            "query": "john.doe@techcorp.com",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.rel("john.doe@techcorp.com").await.unwrap();

    assert_eq!(result.person.full_name, Some("John Doe".to_string()));
    assert_eq!(result.person.email, Some("john.doe@techcorp.com".to_string()));
    assert_eq!(result.person.job_title, Some("Software Engineer".to_string()));
    assert_eq!(result.company.name, Some("TechCorp".to_string()));
    assert_eq!(result.company.domain, Some("techcorp.com".to_string()));
}

#[tokio::test]
async fn test_fcl_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/fcl")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "lookalikes": [
                {
                    "name": "DataCorp",
                    "domain": "datacorp.com",
                    "industry": "Technology"
                },
                {
                    "name": "SoftCorp",
                    "domain": "softcorp.com",
                    "industry": "Technology"
                }
            ],
            "query": "TechCorp",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.fcl("TechCorp").await.unwrap();

    assert_eq!(result.lookalikes.len(), 2);
    assert_eq!(result.lookalikes[0].name, Some("DataCorp".to_string()));
    assert_eq!(result.lookalikes[0].domain, Some("datacorp.com".to_string()));
    assert_eq!(result.lookalikes[1].name, Some("SoftCorp".to_string()));
}

#[tokio::test]
async fn test_elf_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/elf")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "fundraising": {
                "total_funding": 1000000,
                "rounds": 3,
                "last_round": "Series A"
            },
            "query": "TechCorp",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.elf("TechCorp").await.unwrap();

    assert!(result.fundraising.is_object());
}

#[tokio::test]
async fn test_car_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/car")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "revenue": {
                "annual_revenue": 5000000,
                "currency": "USD",
                "year": 2023
            },
            "query": "TechCorp",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.car("TechCorp").await.unwrap();

    assert!(result.revenue.is_object());
}

#[tokio::test]
async fn test_fcc_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/fcc")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "subsidiaries": [
                {
                    "name": "TechCorp Mobile",
                    "domain": "mobile.techcorp.com"
                },
                {
                    "name": "TechCorp Cloud",
                    "domain": "cloud.techcorp.com"
                }
            ],
            "query": "TechCorp",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.fcc("TechCorp").await.unwrap();

    assert_eq!(result.subsidiaries.len(), 2);
    assert_eq!(result.subsidiaries[0].name, Some("TechCorp Mobile".to_string()));
    assert_eq!(result.subsidiaries[0].domain, Some("mobile.techcorp.com".to_string()));
}

#[tokio::test]
async fn test_fts_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/fts")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "tech_stack": {
                "programming_languages": ["Go", "Python", "JavaScript"],
                "frameworks": ["React", "Node.js", "Django"],
                "databases": ["PostgreSQL", "Redis"]
            },
            "query": "TechCorp",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.fts("TechCorp").await.unwrap();

    assert!(result.tech_stack.is_object());
}

#[tokio::test]
async fn test_epp_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/epp")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "person": {
                "full_name": "John Doe",
                "email": "john.doe@techcorp.com",
                "job_title": "Software Engineer",
                "linkedin_url": "https://linkedin.com/in/john-doe"
            },
            "company": {
                "name": "TechCorp",
                "domain": "techcorp.com",
                "industry": "Technology"
            },
            "query": "https://linkedin.com/in/john-doe",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.epp("https://linkedin.com/in/john-doe").await.unwrap();

    assert_eq!(result.person.full_name, Some("John Doe".to_string()));
    assert_eq!(result.person.email, Some("john.doe@techcorp.com".to_string()));
    assert_eq!(result.person.job_title, Some("Software Engineer".to_string()));
    assert_eq!(result.company.name, Some("TechCorp".to_string()));
}

#[tokio::test]
async fn test_fwe_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/fwe")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "email": "john.doe@techcorp.com",
            "query": "https://linkedin.com/in/john-doe",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.fwe("https://linkedin.com/in/john-doe").await.unwrap();

    assert_eq!(result.email, "john.doe@techcorp.com");
}

#[tokio::test]
async fn test_tep_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/tep")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "person": {
                "full_name": "John Doe",
                "job_title": "Software Engineer",
                "company": "TechCorp"
            },
            "query": "John Doe at TechCorp",
            "confidence_level": 88,
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.tep("John Doe", "TechCorp").await.unwrap();

    assert_eq!(result.person.full_name, Some("John Doe".to_string()));
    assert_eq!(result.person.job_title, Some("Software Engineer".to_string()));
    assert_eq!(result.person.company, Some("TechCorp".to_string()));
    assert_eq!(result.base.confidence_level, Some(88));
}

#[tokio::test]
async fn test_enc_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/enc")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "company": {
                "name": "TechCorp",
                "domain": "techcorp.com",
                "industry": "Technology",
                "size": "51-200"
            },
            "query": "TechCorp",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.enc("TechCorp").await.unwrap();

    assert_eq!(result.company.name, Some("TechCorp".to_string()));
    assert_eq!(result.company.domain, Some("techcorp.com".to_string()));
    assert_eq!(result.company.industry, Some("Technology".to_string()));
    assert_eq!(result.company.size, Some("51-200".to_string()));
}

#[tokio::test]
async fn test_cec_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/cec")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "countries": ["US", "UK", "CA"],
            "total_results": 3,
            "query": "TechCorp",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.cec("TechCorp").await.unwrap();

    assert_eq!(result.countries.len(), 3);
    assert!(result.countries.contains(&"US".to_string()));
    assert!(result.countries.contains(&"UK".to_string()));
    assert!(result.countries.contains(&"CA".to_string()));
    assert_eq!(result.total_results, 3);
}

#[tokio::test]
async fn test_clo_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/clo")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "locations": [
                {
                    "country": "US",
                    "state": "CA",
                    "city": "San Francisco",
                    "address": "123 Tech St"
                },
                {
                    "country": "UK",
                    "city": "London",
                    "address": "456 Innovation Ave"
                }
            ],
            "query": "TechCorp",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.clo("TechCorp").await.unwrap();

    assert_eq!(result.locations.len(), 2);
}

#[tokio::test]
async fn test_cse_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/cse")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "companies": [
                {
                    "name": "TechCorp",
                    "domain": "techcorp.com",
                    "industry": "Technology"
                },
                {
                    "name": "DataCorp",
                    "domain": "datacorp.com",
                    "industry": "Data Analytics"
                }
            ],
            "total_results": 2,
            "page": 1,
            "query": "technology",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.cse(CseParams {
        name: Some("technology".to_string()),
        country: Some("US".to_string()),
        industry: Some("Technology".to_string()),
        ..Default::default()
    }).await.unwrap();

    assert_eq!(result.companies.len(), 2);
    assert_eq!(result.companies[0].name, Some("TechCorp".to_string()));
    assert_eq!(result.companies[0].domain, Some("techcorp.com".to_string()));
    assert_eq!(result.companies[0].industry, Some("Technology".to_string()));
    assert_eq!(result.total_results, 2);
    assert_eq!(result.page, 1);
}

#[tokio::test]
async fn test_pse_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/pse")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "people": [
                {
                    "full_name": "John Doe",
                    "job_title": "Software Engineer",
                    "company": "TechCorp"
                },
                {
                    "full_name": "Jane Smith",
                    "job_title": "Product Manager",
                    "company": "TechCorp"
                }
            ],
            "total_results": 2,
            "page": 1,
            "query": "engineer",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.pse(PseParams {
        full_name: Some("engineer".to_string()),
        company_name: Some("TechCorp".to_string()),
        ..Default::default()
    }).await.unwrap();

    assert_eq!(result.people.len(), 2);
    assert_eq!(result.people[0].full_name, Some("John Doe".to_string()));
    assert_eq!(result.people[0].job_title, Some("Software Engineer".to_string()));
    assert_eq!(result.people[0].company, Some("TechCorp".to_string()));
    assert_eq!(result.total_results, 2);
    assert_eq!(result.page, 1);
}

#[tokio::test]
async fn test_lbs_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/lbs")
        .match_header("authorization", "Bearer test-api-key")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "businesses": [
                {
                    "name": "Coffee Shop",
                    "address": "123 Main St",
                    "city": "San Francisco"
                },
                {
                    "name": "Restaurant",
                    "address": "456 Oak Ave",
                    "city": "San Francisco"
                }
            ],
            "total_results": 2,
            "page": 1,
            "query": "coffee",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.lbs(LbsParams {
        name: Some("coffee".to_string()),
        city: Some("San Francisco".to_string()),
        ..Default::default()
    }).await.unwrap();

    assert_eq!(result.businesses.len(), 2);
    assert_eq!(result.businesses[0].name, Some("Coffee Shop".to_string()));
    assert_eq!(result.businesses[0].address, Some("123 Main St".to_string()));
    assert_eq!(result.businesses[0].city, Some("San Francisco".to_string()));
    assert_eq!(result.total_results, 2);
    assert_eq!(result.page, 1);
}

#[tokio::test]
async fn test_error_handling() {
    // Test validation errors
    let mut server = Server::new_async().await;
    let sdk = create_test_sdk(&server.url()).await;
    
    // Test missing required parameters
    let result = sdk.cuf("", "US").await;
    assert!(result.is_err());
    if let Err(CufinderError::ValidationError(msg)) = result {
        assert!(msg.contains("company_name is required"));
    }

    let result = sdk.cuf("TechCorp", "").await;
    assert!(result.is_err());
    if let Err(CufinderError::ValidationError(msg)) = result {
        assert!(msg.contains("country_code is required"));
    }

    let result = sdk.tep("", "TechCorp").await;
    assert!(result.is_err());
    if let Err(CufinderError::ValidationError(msg)) = result {
        assert!(msg.contains("full_name is required"));
    }

    let result = sdk.tep("John Doe", "").await;
    assert!(result.is_err());
    if let Err(CufinderError::ValidationError(msg)) = result {
        assert!(msg.contains("company is required"));
    }
}

#[tokio::test]
async fn test_authentication_error() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/cuf")
        .match_header("authorization", "Bearer test-api-key")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "error": "API key verification failed!"
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.cuf("TechCorp", "US").await;

    assert!(result.is_err());
    if let Err(CufinderError::AuthenticationError(msg)) = result {
        assert!(msg.contains("API key verification failed"));
    }
}

async fn create_test_sdk(base_url: &str) -> CufinderSDK {
    CufinderSDK::with_config(ClientConfig {
        api_key: "test-api-key".to_string(),
        base_url: base_url.to_string(),
        timeout: Duration::from_secs(5),
        max_retries: 1,
    }).unwrap()
}