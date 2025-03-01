use azure_core::error::Result;
use azure_core::headers::{
    account_kind_from_headers, date_from_headers, request_id_from_headers, sku_name_from_headers,
};

use azure_core::RequestId;
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone)]
pub struct GetAccountInformationResponse {
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
    pub sku_name: String,
    pub account_kind: String,
}

impl GetAccountInformationResponse {
    pub(crate) fn from_headers(headers: &HeaderMap) -> Result<GetAccountInformationResponse> {
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;
        let sku_name = sku_name_from_headers(headers)?;
        let account_kind = account_kind_from_headers(headers)?;

        Ok(GetAccountInformationResponse {
            request_id,
            date,
            sku_name,
            account_kind,
        })
    }
}
