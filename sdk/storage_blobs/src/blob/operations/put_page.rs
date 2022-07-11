use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use azure_storage::{headers::content_md5_from_headers, ConsistencyMD5};
use bytes::Bytes;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct PutPageBuilder {
    blob_client: BlobClient,
    ba512_range: BA512Range,
    content: Bytes,
    hash: Option<Hash>,
    sequence_number_condition: Option<SequenceNumberCondition>,
    if_modified_since_condition: Option<IfModifiedSinceCondition>,
    if_match_condition: Option<IfMatchCondition>,
    lease_id: Option<LeaseId>,
    context: Context,
}

impl PutPageBuilder {
    pub(crate) fn new(blob_client: BlobClient, ba512_range: BA512Range, content: Bytes) -> Self {
        Self {
            blob_client,
            ba512_range,
            content,
            hash: None,
            sequence_number_condition: None,
            if_modified_since_condition: None,
            if_match_condition: None,
            lease_id: None,
            context: Context::new(),
        }
    }

    setters! {
        hash: Hash => Some(hash),
        sequence_number_condition: SequenceNumberCondition => Some(sequence_number_condition),
        if_modified_since_condition: IfModifiedSinceCondition => Some(if_modified_since_condition),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        lease_id: LeaseId => Some(lease_id),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;
            url.query_pairs_mut().append_pair("comp", "page");

            let mut headers = Headers::new();
            headers.insert(PAGE_WRITE, "update");
            headers.insert(BLOB_TYPE, "PageBlob");
            headers.add(self.ba512_range);
            headers.add(self.sequence_number_condition);
            headers.add(self.hash);
            headers.add(self.if_modified_since_condition);
            headers.add(self.if_match_condition);
            headers.add(self.lease_id);

            let mut request = self.blob_client.finalize_request(
                url,
                azure_core::Method::Put,
                headers,
                Some(self.content.clone()),
            )?;

            let response = self
                .blob_client
                .send(&mut self.context, &mut request)
                .await?;
            UpdatePageResponse::from_headers(response.headers())
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdatePageResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub content_md5: ConsistencyMD5,
    pub sequence_number: u64,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
    pub request_server_encrypted: bool,
}

impl UpdatePageResponse {
    pub(crate) fn from_headers(headers: &Headers) -> azure_core::Result<UpdatePageResponse> {
        let etag = etag_from_headers(headers)?;
        let last_modified = last_modified_from_headers(headers)?;
        let content_md5 = content_md5_from_headers(headers)?;
        let sequence_number = sequence_number_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;
        let request_server_encrypted = request_server_encrypted_from_headers(headers)?;

        Ok(UpdatePageResponse {
            etag,
            last_modified,
            content_md5,
            sequence_number,
            request_id,
            date,
            request_server_encrypted,
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<UpdatePageResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for PutPageBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}