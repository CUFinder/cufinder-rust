use serde::{Deserialize, Serialize};

/// Base response structure for all CUFinder API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseResponse {
    pub query: Option<serde_json::Value>,
    #[serde(rename = "credit_count")]
    pub credit_count: Option<i32>,
    #[serde(rename = "meta_data")]
    pub meta_data: Option<serde_json::Value>,
    #[serde(rename = "confidence_level")]
    pub confidence_level: Option<i32>,
}

/// Company social media information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanySocial {
    pub facebook: Option<String>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
    pub youtube: Option<String>,
    pub instagram: Option<String>,
}

/// Company employee information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyEmployees {
    pub range: Option<String>,
    pub count: Option<i32>,
}

/// Company main location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainLocation {
    pub geo: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
    pub continent: Option<String>,
    #[serde(rename = "postal_code")]
    pub postal_code: Option<String>,
}

/// Company information model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub name: Option<String>,
    pub domain: Option<String>,
    #[serde(rename = "linkedin_url")]
    pub linkedin_url: Option<String>,
    pub industry: Option<String>,
    pub overview: Option<String>,
    pub r#type: Option<String>,
    pub size: Option<String>,
    #[serde(rename = "main_location")]
    pub main_location: Option<MainLocation>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub founded: Option<i32>,
    pub revenue: Option<String>,
    pub employees: Option<CompanyEmployees>,
    pub website: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub social: Option<CompanySocial>,
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

/// Current job information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentJob {
    pub title: Option<String>,
    pub role: Option<String>,
    pub level: Option<String>,
}


/// Person location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonLocation {
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
}

/// Person social media information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonSocial {
    #[serde(rename = "linkedin_username")]
    pub linkedin_username: Option<String>,
    #[serde(rename = "linkedin_connections")]
    pub linkedin_connections: Option<i32>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
    pub facebook: Option<String>,
    pub github: Option<String>,
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
    pub logo: Option<String>,
    pub overview: Option<String>,
    pub experience: Option<serde_json::Value>,
    pub connections: Option<serde_json::Value>,
    pub interests: Option<Vec<String>>,
    pub skills: Option<Vec<String>>,
    pub educations: Option<Vec<serde_json::Value>>,
    pub experiences: Option<Vec<serde_json::Value>>,
    pub certifications: Option<Vec<serde_json::Value>>,
    pub company: Option<Company>,
    pub location: Option<PersonLocation>,
    #[serde(rename = "current_job")]
    pub current_job: Option<CurrentJob>,
    pub social: Option<PersonSocial>,
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
pub struct RelPerson {
    #[serde(rename = "full_name")]
    pub full_name: Option<String>,
    #[serde(rename = "first_name")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name")]
    pub last_name: Option<String>,
    #[serde(rename = "linkedin_url")]
    pub linkedin_url: Option<String>,
    pub summary: Option<String>,
    #[serde(rename = "linkedin_followers")]
    pub linkedin_followers: Option<String>,
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub avatar: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "job_title")]
    pub job_title: Option<String>,
    #[serde(rename = "job_title_categories")]
    pub job_title_categories: Option<Vec<String>>,
    #[serde(rename = "company_name")]
    pub company_name: Option<String>,
    #[serde(rename = "company_linkedin")]
    pub company_linkedin: Option<String>,
    #[serde(rename = "company_website")]
    pub company_website: Option<String>,
    #[serde(rename = "company_size")]
    pub company_size: Option<String>,
    #[serde(rename = "company_industry")]
    pub company_industry: Option<String>,
    #[serde(rename = "company_facebook")]
    pub company_facebook: Option<String>,
    #[serde(rename = "company_twitter")]
    pub company_twitter: Option<String>,
    #[serde(rename = "company_country")]
    pub company_country: Option<String>,
    #[serde(rename = "company_state")]
    pub company_state: Option<String>,
    #[serde(rename = "company_city")]
    pub company_city: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub person: RelPerson,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FclCompany {
    pub name: Option<String>,
    pub website: Option<String>,
    #[serde(rename = "employee_count")]
    pub employee_count: Option<i32>,
    pub size: Option<String>,
    pub industry: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "linkedin_url")]
    pub linkedin_url: Option<String>,
    pub domain: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
    #[serde(rename = "founded_year")]
    pub founded_year: Option<String>,
    #[serde(rename = "logo_url")]
    pub logo_url: Option<String>,
    #[serde(rename = "followers_count")]
    pub followers_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FclResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub companies: Vec<FclCompany>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElfFundraising {
    #[serde(rename = "funding_last_round_type")]
    pub funding_last_round_type: Option<String>,
    #[serde(rename = "funding_ammount_currency_code")]
    pub funding_ammount_currency_code: Option<String>,
    #[serde(rename = "funding_money_raised")]
    pub funding_money_raised: Option<String>,
    #[serde(rename = "funding_last_round_investors_url")]
    pub funding_last_round_investors_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElfResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(rename = "fundraising_info")]
    pub fundraising: ElfFundraising,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(rename = "annual_revenue")]
    pub revenue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FccResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub subsidiaries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FtsResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub technologies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EppPerson {
    #[serde(rename = "full_name")]
    pub full_name: Option<String>,
    #[serde(rename = "first_name")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name")]
    pub last_name: Option<String>,
    #[serde(rename = "linkedin_url")]
    pub linkedin_url: Option<String>,
    pub summary: Option<String>,
    #[serde(rename = "linkedin_followers")]
    pub linkedin_followers: Option<i32>,
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub avatar: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "job_title")]
    pub job_title: Option<String>,
    #[serde(rename = "job_title_categories")]
    pub job_title_categories: Option<Vec<String>>,
    #[serde(rename = "company_name")]
    pub company_name: Option<String>,
    #[serde(rename = "company_linkedin")]
    pub company_linkedin: Option<String>,
    #[serde(rename = "company_website")]
    pub company_website: Option<String>,
    #[serde(rename = "company_size")]
    pub company_size: Option<String>,
    #[serde(rename = "company_industry")]
    pub company_industry: Option<String>,
    #[serde(rename = "company_facebook")]
    pub company_facebook: Option<String>,
    #[serde(rename = "company_twitter")]
    pub company_twitter: Option<String>,
    #[serde(rename = "company_country")]
    pub company_country: Option<String>,
    #[serde(rename = "company_state")]
    pub company_state: Option<String>,
    #[serde(rename = "company_city")]
    pub company_city: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EppResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub person: EppPerson,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FweResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(rename = "work_email")]
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TepPerson {
    #[serde(rename = "full_name")]
    pub full_name: Option<String>,
    #[serde(rename = "first_name")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name")]
    pub last_name: Option<String>,
    #[serde(rename = "linkedin_url")]
    pub linkedin_url: Option<String>,
    pub summary: Option<String>,
    #[serde(rename = "linkedin_followers")]
    pub linkedin_followers: Option<i32>,
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub avatar: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "job_title")]
    pub job_title: Option<String>,
    #[serde(rename = "job_title_categories")]
    pub job_title_categories: Option<Vec<String>>,
    #[serde(rename = "company_name")]
    pub company_name: Option<String>,
    #[serde(rename = "company_linkedin")]
    pub company_linkedin: Option<String>,
    #[serde(rename = "company_website")]
    pub company_website: Option<String>,
    #[serde(rename = "company_size")]
    pub company_size: Option<String>,
    #[serde(rename = "company_industry")]
    pub company_industry: Option<String>,
    #[serde(rename = "company_facebook")]
    pub company_facebook: Option<String>,
    #[serde(rename = "company_twitter")]
    pub company_twitter: Option<String>,
    #[serde(rename = "company_country")]
    pub company_country: Option<String>,
    #[serde(rename = "company_state")]
    pub company_state: Option<String>,
    #[serde(rename = "company_city")]
    pub company_city: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TepResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub person: TepPerson,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncCompany {
    pub name: Option<String>,
    pub website: Option<String>,
    #[serde(rename = "employee_count")]
    pub employee_count: Option<i32>,
    pub industry: Option<String>,
    pub size: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "linkedin_url")]
    pub linkedin_url: Option<String>,
    pub r#type: Option<String>,
    pub domain: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
    #[serde(rename = "founded_year")]
    pub founded_year: Option<String>,
    #[serde(rename = "logo_url")]
    pub logo_url: Option<String>,
    #[serde(rename = "followers_count")]
    pub followers_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub company: EncCompany,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CecResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub countries: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloLocation {
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "postal_code")]
    pub postal_code: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub locations: Vec<CloLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CseResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub companies: Vec<Company>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PseResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub peoples: Vec<Person>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LbsResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub companies: Vec<Company>,
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
