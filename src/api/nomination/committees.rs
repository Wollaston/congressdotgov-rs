use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::common::Format, api::endpoint::Endpoint, api::params::QueryParams};

/// Represents the /nomination/:congress/:nominationNumber/committees endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Committees {
    #[builder(setter(into))]
    congress: u8,
    #[builder(setter(into))]
    nomination_number: u32,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl Committees {
    pub fn builder() -> CommitteesBuilder {
        CommitteesBuilder::default()
    }
}

impl Endpoint for Committees {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "nomination/{}/{}/committees",
            self.congress, self.nomination_number
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
    use crate::{auth::Auth, cdg::Cdg, api::query::Query};

    use super::*;

    #[test]
    fn is_sufficient() {
        Committees::builder()
            .congress(117_u8)
            .nomination_number(2467_u32)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = Committees::builder()
            .congress(117_u8)
            .nomination_number(2467_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
