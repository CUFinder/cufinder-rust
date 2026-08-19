# Cufinder Rust SDK Changelog


## 1.3.0 (August 16, 2026)

#### Features
- **New services**: Add `PSA` (Contact Signals API), `CSA` (Company Signals API), `JCA` (Job Changes API), `CLF` (Contact Lookalikes API), `NAP` (Person Name Normalizer), `NAU` (URL Normalizer), `GDC` (Gives Demo Checker), and `COT` (Offers Free Trial Checker)


## 1.2.0 (June 21, 2026)

#### Features
- **New services**: Add `CEF` (Company Employee Finder), `NAC` (Company Name Normalizer), `CAA` (Company Activity API), and `CJA` (Company Jobs API)

#### Bug Fixes
- **CJA**: Fix deserialization when response is missing `jobs` field (Not Found case)


## 1.1.0 (February 01, 2026)

#### Features
- **New V2 API services**: Add new V2 services including `BCD`, `CCP`, `ISC`, `CBC`, `CSC`, `CSN`, `NAO` and `NAA`


#### Documentation
- **Updated README.md**: Add API reference for all new services
