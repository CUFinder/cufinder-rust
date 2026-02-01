use crate::{
    client::Client,
    error::{CufinderError, Result},
    types::*,
};

/// Service implementation for CUFinder API
pub struct Service {
    client: Client,
}

impl Service {
    /// Create a new service instance
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// CUF Service - Company URL Finder
    pub async fn get_domain(&self, params: CufParams) -> Result<CufResponse> {
        if params.company_name.is_empty() {
            return Err(CufinderError::ValidationError("company_name is required".to_string()));
        }
        if params.country_code.is_empty() {
            return Err(CufinderError::ValidationError("country_code is required".to_string()));
        }

        let response = self.client.post("/cuf", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// LCUF Service - LinkedIn Company URL Finder
    pub async fn get_linkedin_url(&self, params: LcufParams) -> Result<LcufResponse> {
        if params.company_name.is_empty() {
            return Err(CufinderError::ValidationError("company_name is required".to_string()));
        }

        let response = self.client.post("/lcuf", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// DTC Service - Domain to Company
    pub async fn get_company_name(&self, params: DtcParams) -> Result<DtcResponse> {
        if params.company_website.is_empty() {
            return Err(CufinderError::ValidationError("company_website is required".to_string()));
        }

        let response = self.client.post("/dtc", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// DTE Service - Domain to Emails
    pub async fn get_emails(&self, params: DteParams) -> Result<DteResponse> {
        if params.company_website.is_empty() {
            return Err(CufinderError::ValidationError("company_website is required".to_string()));
        }

        let response = self.client.post("/dte", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// NTP Service - Name to Phones
    pub async fn get_phones(&self, params: NtpParams) -> Result<NtpResponse> {
        if params.company_name.is_empty() {
            return Err(CufinderError::ValidationError("company_name is required".to_string()));
        }

        let response = self.client.post("/ntp", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// REL Service - Reverse Email Lookup
    pub async fn reverse_email_lookup(&self, params: RelParams) -> Result<RelResponse> {
        if params.email.is_empty() {
            return Err(CufinderError::ValidationError("email is required".to_string()));
        }

        let response = self.client.post("/rel", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// FCL Service - Find Company Lookalikes
    pub async fn get_lookalikes(&self, params: FclParams) -> Result<FclResponse> {
        if params.query.is_empty() {
            return Err(CufinderError::ValidationError("query is required".to_string()));
        }

        let response = self.client.post("/fcl", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// ELF Service - Enrich LinkedIn Fundraising
    pub async fn get_fundraising(&self, params: ElfParams) -> Result<ElfResponse> {
        if params.query.is_empty() {
            return Err(CufinderError::ValidationError("query is required".to_string()));
        }

        let response = self.client.post("/elf", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// CAR Service - Company Annual Revenue
    pub async fn get_revenue(&self, params: CarParams) -> Result<CarResponse> {
        if params.query.is_empty() {
            return Err(CufinderError::ValidationError("query is required".to_string()));
        }

        let response = self.client.post("/car", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// FCC Service - Find Company Children
    pub async fn get_subsidiaries(&self, params: FccParams) -> Result<FccResponse> {
        if params.query.is_empty() {
            return Err(CufinderError::ValidationError("query is required".to_string()));
        }

        let response = self.client.post("/fcc", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// FTS Service - Find Tech Stack
    pub async fn get_tech_stack(&self, params: FtsParams) -> Result<FtsResponse> {
        if params.query.is_empty() {
            return Err(CufinderError::ValidationError("query is required".to_string()));
        }

        let response = self.client.post("/fts", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// EPP Service - Enrich Profile
    pub async fn enrich_profile(&self, params: EppParams) -> Result<EppResponse> {
        if params.linkedin_url.is_empty() {
            return Err(CufinderError::ValidationError("linkedin_url is required".to_string()));
        }

        let response = self.client.post("/epp", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// FWE Service - Find Work Email
    pub async fn get_email_from_profile(&self, params: FweParams) -> Result<FweResponse> {
        if params.linkedin_url.is_empty() {
            return Err(CufinderError::ValidationError("linkedin_url is required".to_string()));
        }

        let response = self.client.post("/fwe", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// TEP Service - Person Enrichment
    pub async fn enrich_person(&self, params: TepParams) -> Result<TepResponse> {
        if params.full_name.is_empty() {
            return Err(CufinderError::ValidationError("full_name is required".to_string()));
        }
        if params.company.is_empty() {
            return Err(CufinderError::ValidationError("company is required".to_string()));
        }

        let response = self.client.post("/tep", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// ENC Service - Company Enrichment
    pub async fn enrich_company(&self, params: EncParams) -> Result<EncResponse> {
        if params.query.is_empty() {
            return Err(CufinderError::ValidationError("query is required".to_string()));
        }

        let response = self.client.post("/enc", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// CEC Service - Company Employee Countries
    pub async fn get_employee_countries(&self, params: CecParams) -> Result<CecResponse> {
        if params.query.is_empty() {
            return Err(CufinderError::ValidationError("query is required".to_string()));
        }

        let response = self.client.post("/cec", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// CLO Service - Company Locations
    pub async fn get_locations(&self, params: CloParams) -> Result<CloResponse> {
        if params.query.is_empty() {
            return Err(CufinderError::ValidationError("query is required".to_string()));
        }

        let response = self.client.post("/clo", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// CSE Service - Company Search
    pub async fn search_companies(&self, params: CseParams) -> Result<CseResponse> {
        let response = self.client.post("/cse", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// PSE Service - Person Search
    pub async fn search_people(&self, params: PseParams) -> Result<PseResponse> {
        let response = self.client.post("/pse", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// LBS Service - Local Business Search
    pub async fn search_local_businesses(&self, params: LbsParams) -> Result<LbsResponse> {
        let response = self.client.post("/lbs", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// BCD - B2B Customers Finder
    pub async fn extract_b2b_customers(&self, params: BcdParams) -> Result<BcdResponse> {
        if params.url.is_empty() {
            return Err(CufinderError::ValidationError("url is required".to_string()));
        }

        let response = self.client.post("/bcd", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// CCP - Company Career Page Finder
    pub async fn find_company_careers_page(&self, params: CcpParams) -> Result<CcpResponse> {
        if params.url.is_empty() {
            return Err(CufinderError::ValidationError("url is required".to_string()));
        }

        let response = self.client.post("/ccp", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }

    /// ISC - Company Saas Checker
    pub async fn is_saas(&self, params: IscParams) -> Result<IscResponse> {
        if params.url.is_empty() {
            return Err(CufinderError::ValidationError("url is required".to_string()));
        }

        let response = self.client.post("/isc", &params).await?;
        serde_json::from_value(response).map_err(CufinderError::JsonError)
    }
}
