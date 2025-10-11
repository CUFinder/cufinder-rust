use serde::{Deserialize, Serialize};

/// Base response structure for all CUFinder API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseResponse {
    pub query: Option<String>,
    #[serde(rename = "credit_count")]
    pub credit_count: Option<i32>,
    #[serde(rename = "meta_data")]
    pub meta_data: Option<serde_json::Value>,
    #[serde(rename = "confidence_level")]
    pub confidence_level: Option<i32>,
}

/// Company information model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub name: Option<String>,
    pub domain: Option<String>,
    #[serde(rename = "linkedin_url")]
    pub linkedin_url: Option<String>,
    pub industry: Option<String>,
    pub size: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub founded: Option<i32>,
    pub revenue: Option<String>,
    pub employees: Option<serde_json::Value>,
    pub website: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "social_media")]
    pub social_media: Option<serde_json::Value>,
    pub technologies: Option<Vec<String>>,
    pub subsidiaries: Option<Vec<String>>,
    pub headquarters: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "zip_code")]
    pub zip_code: Option<String>,
    pub address: Option<String>,
}

/// Person information model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    #[serde(rename = "first_name")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name")]
    pub last_name: Option<String>,
    #[serde(rename = "full_name")]
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    #[serde(rename = "linkedin_url")]
    pub linkedin_url: Option<String>,
    #[serde(rename = "job_title")]
    pub job_title: Option<String>,
    pub company: Option<String>,
    #[serde(rename = "company_domain")]
    pub company_domain: Option<String>,
    pub location: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub bio: Option<String>,
    pub experience: Option<Vec<serde_json::Value>>,
    pub education: Option<Vec<serde_json::Value>>,
    pub skills: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    #[serde(rename = "social_media")]
    pub social_media: Option<serde_json::Value>,
}

// Response types for each service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CufResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LcufResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(rename = "linkedin_url")]
    pub linkedin_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DtcResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(rename = "company_name")]
    pub company_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DteResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub emails: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NtpResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub phones: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub person: Person,
    pub company: Company,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FclResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub lookalikes: Vec<Company>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElfResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub fundraising: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub revenue: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FccResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub subsidiaries: Vec<Company>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FtsResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(rename = "tech_stack")]
    pub tech_stack: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EppResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub person: Person,
    pub company: Company,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FweResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TepResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub person: Person,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub company: Company,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CecResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub countries: Vec<String>,
    #[serde(rename = "total_results")]
    pub total_results: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub locations: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CseResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub companies: Vec<Company>,
    #[serde(rename = "total_results")]
    pub total_results: i32,
    pub page: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PseResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub people: Vec<Person>,
    #[serde(rename = "total_results")]
    pub total_results: i32,
    pub page: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LbsResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub businesses: Vec<Company>,
    #[serde(rename = "total_results")]
    pub total_results: i32,
    pub page: i32,
}

// Parameter types for each service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CufParams {
    #[serde(rename = "company_name")]
    pub company_name: String,
    #[serde(rename = "country_code")]
    pub country_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LcufParams {
    #[serde(rename = "company_name")]
    pub company_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DtcParams {
    #[serde(rename = "company_website")]
    pub company_website: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DteParams {
    #[serde(rename = "company_website")]
    pub company_website: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NtpParams {
    #[serde(rename = "company_name")]
    pub company_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelParams {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FclParams {
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElfParams {
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarParams {
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FccParams {
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FtsParams {
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EppParams {
    #[serde(rename = "linkedin_url")]
    pub linkedin_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FweParams {
    #[serde(rename = "linkedin_url")]
    pub linkedin_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TepParams {
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub company: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncParams {
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CecParams {
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloParams {
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CseParams {
    pub name: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "followers_count_min")]
    pub followers_count_min: Option<i32>,
    #[serde(rename = "followers_count_max")]
    pub followers_count_max: Option<i32>,
    pub industry: Option<String>,
    #[serde(rename = "employee_size")]
    pub employee_size: Option<String>,
    #[serde(rename = "founded_after_year")]
    pub founded_after_year: Option<i32>,
    #[serde(rename = "founded_before_year")]
    pub founded_before_year: Option<i32>,
    #[serde(rename = "funding_amount_max")]
    pub funding_amount_max: Option<i32>,
    #[serde(rename = "funding_amount_min")]
    pub funding_amount_min: Option<i32>,
    #[serde(rename = "products_services")]
    pub products_services: Option<Vec<String>>,
    #[serde(rename = "is_school")]
    pub is_school: Option<bool>,
    #[serde(rename = "annual_revenue_min")]
    pub annual_revenue_min: Option<i32>,
    #[serde(rename = "annual_revenue_max")]
    pub annual_revenue_max: Option<i32>,
    pub page: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PseParams {
    #[serde(rename = "full_name")]
    pub full_name: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "job_title_role")]
    pub job_title_role: Option<String>,
    #[serde(rename = "job_title_level")]
    pub job_title_level: Option<String>,
    #[serde(rename = "company_country")]
    pub company_country: Option<String>,
    #[serde(rename = "company_state")]
    pub company_state: Option<String>,
    #[serde(rename = "company_city")]
    pub company_city: Option<String>,
    #[serde(rename = "company_name")]
    pub company_name: Option<String>,
    #[serde(rename = "company_linkedin_url")]
    pub company_linkedin_url: Option<String>,
    #[serde(rename = "company_industry")]
    pub company_industry: Option<String>,
    #[serde(rename = "company_employee_size")]
    pub company_employee_size: Option<String>,
    #[serde(rename = "company_products_services")]
    pub company_products_services: Option<Vec<String>>,
    #[serde(rename = "company_annual_revenue_min")]
    pub company_annual_revenue_min: Option<i32>,
    #[serde(rename = "company_annual_revenue_max")]
    pub company_annual_revenue_max: Option<i32>,
    pub page: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LbsParams {
    pub name: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub industry: Option<String>,
    pub page: Option<i32>,
}
