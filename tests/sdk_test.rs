use cufinder_rust::{
    CufinderSDK, ClientConfig, CseParams, PseParams, LbsParams,
    PsaParams, CsaParams, JcaParams, ClfParams, NapParams, NauParams, GdcParams, CotParams,
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
        .match_header("x-api-key", "test-api-key")
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
    assert_eq!(result.base.query, Some(json!("TechCorp")));
    assert_eq!(result.base.credit_count, Some(1));
}

#[tokio::test]
async fn test_lcuf_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/lcuf")
        .match_header("x-api-key", "test-api-key")
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
    assert_eq!(result.base.query, Some(json!("TechCorp")));
}

#[tokio::test]
async fn test_dtc_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/dtc")
        .match_header("x-api-key", "test-api-key")
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
    assert_eq!(result.base.query, Some(json!("techcorp.com")));
}

#[tokio::test]
async fn test_dte_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/dte")
        .match_header("x-api-key", "test-api-key")
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
        .match_header("x-api-key", "test-api-key")
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
        .match_header("x-api-key", "test-api-key")
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
    assert_eq!(result.person.job_title, Some("Software Engineer".to_string()));
}

#[tokio::test]
async fn test_fcl_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/fcl")
        .match_header("x-api-key", "test-api-key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "companies": [
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

    assert_eq!(result.companies.len(), 2);
    assert_eq!(result.companies[0].name, Some("DataCorp".to_string()));
    assert_eq!(result.companies[0].domain, Some("datacorp.com".to_string()));
    assert_eq!(result.companies[1].name, Some("SoftCorp".to_string()));
}

#[tokio::test]
async fn test_elf_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/elf")
        .match_header("x-api-key", "test-api-key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "fundraising_info": {
                "funding_last_round_type": "Series A",
                "funding_ammount_currency_code": "USD",
                "funding_money_raised": "1000000"
            },
            "query": "TechCorp",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.elf("TechCorp").await.unwrap();

    assert_eq!(result.fundraising.funding_last_round_type, Some("Series A".to_string()));
}

#[tokio::test]
async fn test_car_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/car")
        .match_header("x-api-key", "test-api-key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "annual_revenue": "$5M",
            "query": "TechCorp",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.car("TechCorp").await.unwrap();

    assert_eq!(result.revenue, "$5M");
}

#[tokio::test]
async fn test_fcc_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/fcc")
        .match_header("x-api-key", "test-api-key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "subsidiaries": ["TechCorp Mobile", "TechCorp Cloud"],
            "query": "TechCorp",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.fcc("TechCorp").await.unwrap();

    assert_eq!(result.subsidiaries.len(), 2);
    assert_eq!(result.subsidiaries[0], "TechCorp Mobile");
    assert_eq!(result.subsidiaries[1], "TechCorp Cloud");
}

#[tokio::test]
async fn test_fts_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/fts")
        .match_header("x-api-key", "test-api-key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "technologies": ["Go", "Python", "JavaScript"],
            "query": "TechCorp",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.fts("TechCorp").await.unwrap();

    assert_eq!(result.technologies, vec!["Go".to_string(), "Python".to_string(), "JavaScript".to_string()]);
}

#[tokio::test]
async fn test_epp_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/epp")
        .match_header("x-api-key", "test-api-key")
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
    assert_eq!(result.person.job_title, Some("Software Engineer".to_string()));
}

#[tokio::test]
async fn test_fwe_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/fwe")
        .match_header("x-api-key", "test-api-key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "work_email": "john.doe@techcorp.com",
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
        .match_header("x-api-key", "test-api-key")
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
    assert_eq!(result.base.confidence_level, Some(88));
}

#[tokio::test]
async fn test_enc_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/enc")
        .match_header("x-api-key", "test-api-key")
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
        .match_header("x-api-key", "test-api-key")
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

    assert_eq!(result.countries.as_array().unwrap().len(), 3);
    assert!(result.countries.as_array().unwrap().contains(&json!("US")));
    assert!(result.countries.as_array().unwrap().contains(&json!("UK")));
    assert!(result.countries.as_array().unwrap().contains(&json!("CA")));
}

#[tokio::test]
async fn test_clo_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/clo")
        .match_header("x-api-key", "test-api-key")
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
        .match_header("x-api-key", "test-api-key")
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
}

#[tokio::test]
async fn test_pse_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/pse")
        .match_header("x-api-key", "test-api-key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "peoples": [
                {
                    "full_name": "John Doe",
                    "current_job": { "title": "Software Engineer" },
                    "company": { "name": "TechCorp" }
                },
                {
                    "full_name": "Jane Smith",
                    "current_job": { "title": "Product Manager" },
                    "company": { "name": "TechCorp" }
                }
            ],
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

    assert_eq!(result.peoples.len(), 2);
    assert_eq!(result.peoples[0].full_name, Some("John Doe".to_string()));
    assert_eq!(result.peoples[0].current_job.as_ref().unwrap().title, Some("Software Engineer".to_string()));
    assert_eq!(result.peoples[0].company.as_ref().unwrap().name, Some("TechCorp".to_string()));
}

#[tokio::test]
async fn test_lbs_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/lbs")
        .match_header("x-api-key", "test-api-key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "companies": [
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

    assert_eq!(result.companies.len(), 2);
    assert_eq!(result.companies[0].name, Some("Coffee Shop".to_string()));
    assert_eq!(result.companies[0].address, Some("123 Main St".to_string()));
    assert_eq!(result.companies[0].city, Some("San Francisco".to_string()));
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
        .match_header("x-api-key", "test-api-key")
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

#[tokio::test]
async fn test_cef_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/cef")
        .match_header("x-api-key", "test-api-key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "employees": [
                {
                    "full_name": "John Doe",
                    "first_name": "John",
                    "last_name": "Doe",
                    "linkedin_url": "https://linkedin.com/in/john-doe",
                    "job_title": "Software Engineer",
                    "company_name": "TechCorp",
                    "company_industry": "Technology",
                    "country": "US",
                    "state": "CA",
                    "city": "San Francisco"
                },
                {
                    "full_name": "Jane Smith",
                    "first_name": "Jane",
                    "last_name": "Smith",
                    "linkedin_url": "https://linkedin.com/in/jane-smith",
                    "job_title": "Product Manager",
                    "company_name": "TechCorp",
                    "company_industry": "Technology",
                    "country": "US",
                    "state": "CA",
                    "city": "San Francisco"
                }
            ],
            "query": "TechCorp",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.cef("TechCorp", None).await.unwrap();

    assert_eq!(result.employees.len(), 2);
    assert_eq!(result.employees[0].full_name, Some("John Doe".to_string()));
    assert_eq!(result.employees[0].first_name, Some("John".to_string()));
    assert_eq!(result.employees[0].last_name, Some("Doe".to_string()));
    assert_eq!(result.employees[0].job_title, Some("Software Engineer".to_string()));
    assert_eq!(result.employees[0].company_name, Some("TechCorp".to_string()));
    assert_eq!(result.employees[0].country, Some("US".to_string()));
    assert_eq!(result.employees[0].state, Some("CA".to_string()));
    assert_eq!(result.employees[0].city, Some("San Francisco".to_string()));
    assert_eq!(result.employees[1].full_name, Some("Jane Smith".to_string()));
    assert_eq!(result.employees[1].job_title, Some("Product Manager".to_string()));
    assert_eq!(result.base.query, Some(json!("TechCorp")));
    assert_eq!(result.base.credit_count, Some(1));
}

#[tokio::test]
async fn test_cef_service_with_page() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/cef")
        .match_header("x-api-key", "test-api-key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "employees": [
                {
                    "full_name": "Bob Johnson",
                    "first_name": "Bob",
                    "last_name": "Johnson",
                    "job_title": "DevOps Engineer",
                    "company_name": "TechCorp"
                }
            ],
            "query": "TechCorp",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.cef("TechCorp", Some(2)).await.unwrap();

    assert_eq!(result.employees.len(), 1);
    assert_eq!(result.employees[0].full_name, Some("Bob Johnson".to_string()));
}

#[tokio::test]
async fn test_cef_validation_error() {
    let mut server = Server::new_async().await;
    let sdk = create_test_sdk(&server.url()).await;

    let result = sdk.cef("", None).await;
    assert!(result.is_err());
    if let Err(CufinderError::ValidationError(msg)) = result {
        assert!(msg.contains("query is required"));
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
#[tokio::test]
async fn test_psa_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/psa")
        .match_header("x-api-key", "test-api-key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "contacts": [
                {
                    "full_name": "John Doe",
                    "current_job": { "title": "Software Engineer" },
                    "company": {
                        "name": "TechCorp",
                        "linkedin": "linkedin.com/company/techcorp",
                        "website": "https://techcorp.com",
                        "industry": "software development",
                        "main_location": {
                            "country": "united states",
                            "state": "california",
                            "city": "san francisco"
                        }
                    },
                    "location": {
                        "country": "united states",
                        "state": "california",
                        "city": "san francisco"
                    },
                    "signal": {
                        "name": "employee_growth",
                        "time_frame": 90,
                        "bucket": "high"
                    }
                }
            ],
            "query": {
                "signal_name": "employee_growth",
                "time_frame": 90,
                "bucket": "high",
                "page": 1
            },
            "credit_count": 1,
            "meta_data": { "total_results": 1 }
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.psa(PsaParams {
        signal_name: "employee_growth".to_string(),
        time_frame: Some(90),
        bucket: "high".to_string(),
        page: Some(1),
    }).await.unwrap();

    assert_eq!(result.contacts.len(), 1);
    assert_eq!(result.contacts[0].full_name, Some("John Doe".to_string()));
    assert_eq!(result.contacts[0].signal.as_ref().unwrap().name, Some("employee_growth".to_string()));
    assert_eq!(result.contacts[0].signal.as_ref().unwrap().time_frame, Some(90));
    assert_eq!(result.contacts[0].signal.as_ref().unwrap().bucket, Some("high".to_string()));
    assert_eq!(result.base.credit_count, Some(1));
}

#[tokio::test]
async fn test_csa_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/csa")
        .match_header("x-api-key", "test-api-key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "companies": [
                {
                    "name": "TechCorp",
                    "website": "https://techcorp.com",
                    "domain": "techcorp.com",
                    "industry": "software development",
                    "overview": "Enterprise software company",
                    "type": "private",
                    "employees": { "range": "1001-5000" },
                    "main_location": {
                        "country": "united states",
                        "state": "california",
                        "city": "san francisco",
                        "address": "123 Tech St"
                    },
                    "signal": {
                        "name": "employee_growth",
                        "time_frame": 90,
                        "bucket": "high"
                    }
                }
            ],
            "query": {
                "signal_name": "employee_growth",
                "time_frame": 90,
                "bucket": "high",
                "page": 1
            },
            "credit_count": 1,
            "meta_data": { "total_results": 1 }
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.csa(CsaParams {
        signal_name: "employee_growth".to_string(),
        time_frame: Some(90),
        bucket: "high".to_string(),
        page: Some(1),
    }).await.unwrap();

    assert_eq!(result.companies.len(), 1);
    assert_eq!(result.companies[0].name, Some("TechCorp".to_string()));
    assert_eq!(result.companies[0].domain, Some("techcorp.com".to_string()));
    assert_eq!(result.companies[0].industry, Some("software development".to_string()));
    assert_eq!(result.companies[0].signal.as_ref().unwrap().name, Some("employee_growth".to_string()));
    assert_eq!(result.base.credit_count, Some(1));
}

#[tokio::test]
async fn test_jca_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/jca")
        .match_header("x-api-key", "test-api-key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "job_changes": [
                {
                    "type": "promotion",
                    "linkedin_url": "https://linkedin.com/in/john-doe",
                    "detected_at": "2026-08-01T12:00:00Z",
                    "from": {
                        "company_linkedin_url": "https://linkedin.com/company/techcorp",
                        "company_linkedin_id": "12345",
                        "company_name": "TechCorp",
                        "title": "Software Engineer"
                    },
                    "to": {
                        "company_linkedin_url": "https://linkedin.com/company/techcorp",
                        "company_linkedin_id": "12345",
                        "company_name": "TechCorp",
                        "title": "Senior Software Engineer"
                    }
                }
            ],
            "query": {
                "start_date": "2026-01-01",
                "end_date": "2026-08-16",
                "type": "promotion"
            },
            "credit_count": 1,
            "meta_data": { "total_results": 1 }
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.jca(JcaParams {
        start_date: "2026-01-01".to_string(),
        end_date: "2026-08-16".to_string(),
        r#type: Some("promotion".to_string()),
        page: None,
    }).await.unwrap();

    assert_eq!(result.job_changes.len(), 1);
    assert_eq!(result.job_changes[0].r#type, Some("promotion".to_string()));
    assert_eq!(result.job_changes[0].from.as_ref().unwrap().company_name, Some("TechCorp".to_string()));
    assert_eq!(result.job_changes[0].from.as_ref().unwrap().title, Some("Software Engineer".to_string()));
    assert_eq!(result.job_changes[0].to.as_ref().unwrap().title, Some("Senior Software Engineer".to_string()));
    assert_eq!(result.base.credit_count, Some(1));
}

#[tokio::test]
async fn test_clf_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/clf")
        .match_header("x-api-key", "test-api-key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "profiles": [
                {
                    "full_name": "Morteza Heydari",
                    "linkedin_url": "https://linkedin.com/in/mortezaheydari1997",
                    "job_title": "Founder & CEO",
                    "company_name": "CUFinder",
                    "country": "united states",
                    "state": "new york",
                    "city": "new york"
                }
            ],
            "query": "linkedin.com/in/mortezaheydari1997",
            "credit_count": 1,
            "meta_data": { "total_results": 1 }
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.clf(ClfParams {
        query: "linkedin.com/in/mortezaheydari1997".to_string(),
    }).await.unwrap();

    assert_eq!(result.profiles.len(), 1);
    assert_eq!(result.profiles[0].full_name, Some("Morteza Heydari".to_string()));
    assert_eq!(result.profiles[0].linkedin_url, Some("https://linkedin.com/in/mortezaheydari1997".to_string()));
    assert_eq!(result.profiles[0].job_title, Some("Founder & CEO".to_string()));
    assert_eq!(result.profiles[0].company_name, Some("CUFinder".to_string()));
    assert_eq!(result.base.credit_count, Some(1));
}

#[tokio::test]
async fn test_nap_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/nap")
        .match_header("x-api-key", "test-api-key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "normalized_name": "Morteza Heydari",
            "query": "morteza heydari",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.nap(NapParams {
        person_name: "morteza heydari".to_string(),
    }).await.unwrap();

    assert_eq!(result.normalized_name, Some("Morteza Heydari".to_string()));
    assert_eq!(result.base.credit_count, Some(1));
}

#[tokio::test]
async fn test_nau_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/nau")
        .match_header("x-api-key", "test-api-key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "normalized_url": "https://www.cufinder.io/about-us",
            "query": "https://www.cufinder.io/about-us",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.nau(NauParams {
        url: "https://www.cufinder.io/about-us".to_string(),
    }).await.unwrap();

    assert_eq!(result.normalized_url, Some("https://www.cufinder.io/about-us".to_string()));
    assert_eq!(result.base.credit_count, Some(1));
}

#[tokio::test]
async fn test_gdc_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/gdc")
        .match_header("x-api-key", "test-api-key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "offers_demo": "yes",
            "query": "https://www.stripe.com",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.gdc(GdcParams {
        url: "https://www.stripe.com".to_string(),
    }).await.unwrap();

    assert_eq!(result.offers_demo, Some("yes".to_string()));
    assert_eq!(result.base.credit_count, Some(1));
}

#[tokio::test]
async fn test_cot_service() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/cot")
        .match_header("x-api-key", "test-api-key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "offers_free_trial": "yes",
            "query": "https://www.stripe.com",
            "credit_count": 1
        }).to_string())
        .create();

    let sdk = create_test_sdk(&server.url()).await;
    let result = sdk.cot(CotParams {
        url: "https://www.stripe.com".to_string(),
    }).await.unwrap();

    assert_eq!(result.offers_free_trial, Some("yes".to_string()));
    assert_eq!(result.base.credit_count, Some(1));
}

#[tokio::test]
async fn test_psa_validation_error() {
    let mut server = Server::new_async().await;
    let sdk = create_test_sdk(&server.url()).await;

    let result = sdk.psa(PsaParams {
        signal_name: "".to_string(),
        time_frame: Some(90),
        bucket: "high".to_string(),
        page: None,
    }).await;
    assert!(result.is_err());
    if let Err(CufinderError::ValidationError(msg)) = result {
        assert!(msg.contains("signal_name is required"));
    }

    let result = sdk.psa(PsaParams {
        signal_name: "employee_growth".to_string(),
        time_frame: Some(90),
        bucket: "".to_string(),
        page: None,
    }).await;
    assert!(result.is_err());
    if let Err(CufinderError::ValidationError(msg)) = result {
        assert!(msg.contains("bucket is required"));
    }
}

#[tokio::test]
async fn test_jca_validation_error() {
    let mut server = Server::new_async().await;
    let sdk = create_test_sdk(&server.url()).await;

    let result = sdk.jca(JcaParams {
        start_date: "".to_string(),
        end_date: "2026-08-16".to_string(),
        r#type: None,
        page: None,
    }).await;
    assert!(result.is_err());
    if let Err(CufinderError::ValidationError(msg)) = result {
        assert!(msg.contains("start_date is required"));
    }

    let result = sdk.jca(JcaParams {
        start_date: "2026-01-01".to_string(),
        end_date: "".to_string(),
        r#type: None,
        page: None,
    }).await;
    assert!(result.is_err());
    if let Err(CufinderError::ValidationError(msg)) = result {
        assert!(msg.contains("end_date is required"));
    }
}

#[tokio::test]
async fn test_clf_validation_error() {
    let mut server = Server::new_async().await;
    let sdk = create_test_sdk(&server.url()).await;

    let result = sdk.clf(ClfParams {
        query: "".to_string(),
    }).await;
    assert!(result.is_err());
    if let Err(CufinderError::ValidationError(msg)) = result {
        assert!(msg.contains("query is required"));
    }
}

#[tokio::test]
async fn test_nap_validation_error() {
    let mut server = Server::new_async().await;
    let sdk = create_test_sdk(&server.url()).await;

    let result = sdk.nap(NapParams {
        person_name: "".to_string(),
    }).await;
    assert!(result.is_err());
    if let Err(CufinderError::ValidationError(msg)) = result {
        assert!(msg.contains("person_name is required"));
    }
}

#[tokio::test]
async fn test_nau_validation_error() {
    let mut server = Server::new_async().await;
    let sdk = create_test_sdk(&server.url()).await;

    let result = sdk.nau(NauParams {
        url: "".to_string(),
    }).await;
    assert!(result.is_err());
    if let Err(CufinderError::ValidationError(msg)) = result {
        assert!(msg.contains("url is required"));
    }
}

#[tokio::test]
async fn test_gdc_validation_error() {
    let mut server = Server::new_async().await;
    let sdk = create_test_sdk(&server.url()).await;

    let result = sdk.gdc(GdcParams {
        url: "".to_string(),
    }).await;
    assert!(result.is_err());
    if let Err(CufinderError::ValidationError(msg)) = result {
        assert!(msg.contains("url is required"));
    }
}

#[tokio::test]
async fn test_cot_validation_error() {
    let mut server = Server::new_async().await;
    let sdk = create_test_sdk(&server.url()).await;

    let result = sdk.cot(CotParams {
        url: "".to_string(),
    }).await;
    assert!(result.is_err());
    if let Err(CufinderError::ValidationError(msg)) = result {
        assert!(msg.contains("url is required"));
    }
}
