use crate::{
    client::{Client, ClientConfig},
    error::Result,
    services::Service,
    types::*,
};
use std::time::Duration;

/// Main CUFinder SDK
pub struct CufinderSDK {
    client: Client,
    service: Service,
}

impl CufinderSDK {
    /// Create a new SDK instance with just an API key
    pub fn new(api_key: String) -> Result<Self> {
        let config = ClientConfig {
            api_key,
            base_url: "https://api.cufinder.io/v2".to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
        };

        Self::with_config(config)
    }

    /// Create a new SDK instance with custom configuration
    pub fn with_config(config: ClientConfig) -> Result<Self> {
        let client = Client::new(config)?;
        let service = Service::new(client.clone());

        Ok(Self { client, service })
    }

    /// Get the underlying HTTP client for advanced usage
    pub fn client(&self) -> &Client {
        &self.client
    }

    // Company Services

    /// CUF - Get company domain from company name
    pub async fn cuf(&self, company_name: &str, country_code: &str) -> Result<CufResponse> {
        self.service.get_domain(CufParams {
            company_name: company_name.to_string(),
            country_code: country_code.to_string(),
        }).await
    }

    /// LCUF - Get LinkedIn URL from company name
    pub async fn lcuf(&self, company_name: &str) -> Result<LcufResponse> {
        self.service.get_linkedin_url(LcufParams {
            company_name: company_name.to_string(),
        }).await
    }

    /// DTC - Get company name from domain
    pub async fn dtc(&self, company_website: &str) -> Result<DtcResponse> {
        self.service.get_company_name(DtcParams {
            company_website: company_website.to_string(),
        }).await
    }

    /// DTE - Get company emails from domain
    pub async fn dte(&self, company_website: &str) -> Result<DteResponse> {
        self.service.get_emails(DteParams {
            company_website: company_website.to_string(),
        }).await
    }

    /// NTP - Get company phones from company name
    pub async fn ntp(&self, company_name: &str) -> Result<NtpResponse> {
        self.service.get_phones(NtpParams {
            company_name: company_name.to_string(),
        }).await
    }

    // Person Services

    /// EPP - Enrich LinkedIn profile
    pub async fn epp(&self, linkedin_url: &str) -> Result<EppResponse> {
        self.service.enrich_profile(EppParams {
            linkedin_url: linkedin_url.to_string(),
        }).await
    }

    /// REL - Reverse email lookup
    pub async fn rel(&self, email: &str) -> Result<RelResponse> {
        self.service.reverse_email_lookup(RelParams {
            email: email.to_string(),
        }).await
    }

    /// FWE - Get email from profile
    pub async fn fwe(&self, linkedin_url: &str) -> Result<FweResponse> {
        self.service.get_email_from_profile(FweParams {
            linkedin_url: linkedin_url.to_string(),
        }).await
    }

    /// TEP - Enrich person information
    pub async fn tep(&self, full_name: &str, company: &str) -> Result<TepResponse> {
        self.service.enrich_person(TepParams {
            full_name: full_name.to_string(),
            company: company.to_string(),
        }).await
    }

    // Company Intelligence Services

    /// FCL - Get company lookalikes
    pub async fn fcl(&self, query: &str) -> Result<FclResponse> {
        self.service.get_lookalikes(FclParams {
            query: query.to_string(),
        }).await
    }

    /// ELF - Get company fundraising information
    pub async fn elf(&self, query: &str) -> Result<ElfResponse> {
        self.service.get_fundraising(ElfParams {
            query: query.to_string(),
        }).await
    }

    /// CAR - Get company revenue
    pub async fn car(&self, query: &str) -> Result<CarResponse> {
        self.service.get_revenue(CarParams {
            query: query.to_string(),
        }).await
    }

    /// FCC - Get company subsidiaries
    pub async fn fcc(&self, query: &str) -> Result<FccResponse> {
        self.service.get_subsidiaries(FccParams {
            query: query.to_string(),
        }).await
    }

    /// FTS - Get company tech stack
    pub async fn fts(&self, query: &str) -> Result<FtsResponse> {
        self.service.get_tech_stack(FtsParams {
            query: query.to_string(),
        }).await
    }

    /// ENC - Enrich company information
    pub async fn enc(&self, query: &str) -> Result<EncResponse> {
        self.service.enrich_company(EncParams {
            query: query.to_string(),
        }).await
    }

    /// CEC - Get company employee countries
    pub async fn cec(&self, query: &str) -> Result<CecResponse> {
        self.service.get_employee_countries(CecParams {
            query: query.to_string(),
        }).await
    }

    /// CLO - Get company locations
    pub async fn clo(&self, query: &str) -> Result<CloResponse> {
        self.service.get_locations(CloParams {
            query: query.to_string(),
        }).await
    }

    // Search Services

    /// CSE - Search companies
    pub async fn cse(&self, params: CseParams) -> Result<CseResponse> {
        self.service.search_companies(params).await
    }

    /// PSE - Search people
    pub async fn pse(&self, params: PseParams) -> Result<PseResponse> {
        self.service.search_people(params).await
    }

    /// LBS - Search local businesses
    pub async fn lbs(&self, params: LbsParams) -> Result<LbsResponse> {
        self.service.search_local_businesses(params).await
    }

    /// BCD - B2B Customers Finder
    pub async fn bcd(&self, url: &str) -> Result<BcdResponse> {
        self.service.extract_b2b_customers(BcdParams {
            url: url.to_string(),
        }).await
    }

    /// CCP - Company Career Page Finder
    pub async fn ccp(&self, url: &str) -> Result<CcpResponse> {
        self.service.find_company_careers_page(CcpParams {
            url: url.to_string(),
        }).await
    }
}
