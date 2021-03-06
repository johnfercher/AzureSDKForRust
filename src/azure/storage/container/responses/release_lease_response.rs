use azure::core::errors::AzureError;
use azure::core::headers;
use azure::core::RequestId;
use chrono::{DateTime, FixedOffset};
use http::header;
use http::HeaderMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct ReleaseLeaseResponse {
    pub etag: String,
    pub last_modified: DateTime<FixedOffset>,
    pub request_id: RequestId,
    pub date: DateTime<FixedOffset>,
}

impl ReleaseLeaseResponse {
    pub(crate) fn from_response(headers: &HeaderMap) -> Result<ReleaseLeaseResponse, AzureError> {
        let etag = match headers.get(header::ETAG) {
            Some(etag) => etag.to_str()?.to_owned(),
            None => return Err(AzureError::MissingHeaderError(header::ETAG.as_str().to_owned())),
        };

        let last_modified = match headers.get(header::LAST_MODIFIED) {
            Some(last_modified) => last_modified.to_str()?,
            None => return Err(AzureError::MissingHeaderError(header::LAST_MODIFIED.as_str().to_owned())),
        };
        let last_modified = DateTime::parse_from_rfc2822(last_modified)?;

        let request_id = match headers.get(headers::REQUEST_ID) {
            Some(request_id) => request_id.to_str()?,
            None => return Err(AzureError::MissingHeaderError(headers::REQUEST_ID.to_owned())),
        };
        let request_id = Uuid::parse_str(request_id)?;

        let date = match headers.get(header::DATE) {
            Some(date) => date.to_str()?,
            None => return Err(AzureError::MissingHeaderError(header::DATE.as_str().to_owned())),
        };
        let date = DateTime::parse_from_rfc2822(date)?;

        Ok(ReleaseLeaseResponse {
            etag,
            last_modified,
            request_id,
            date,
        })
    }
}
