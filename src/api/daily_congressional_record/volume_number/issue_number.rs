use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{endpoint::Endpoint, params::QueryParams};

use super::Format;

mod articles;

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct IssueNumber {
    #[builder(setter(into))]
    volume_number: u32,
    #[builder(setter(into))]
    issue_number: u32,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl IssueNumber {
    pub fn builder() -> IssueNumberBuilder {
        IssueNumberBuilder::default()
    }
}

impl Endpoint for IssueNumber {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "daily-congressional-record/{}/{}",
            self.volume_number, self.issue_number
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("format", self.format);
        params.push_opt("offset", self.offset);
        params.push_opt("limit", self.limit);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::daily_congressional_record::volume_number::issue_number::IssueNumber, auth::Auth,
        cdg::Cdg, query::Query,
    };

    #[test]
    fn is_sufficient() {
        IssueNumber::builder()
            .volume_number(168_u32)
            .issue_number(153_u32)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = IssueNumber::builder()
            .volume_number(168_u32)
            .issue_number(153_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
