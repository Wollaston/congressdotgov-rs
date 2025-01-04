use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::common::Format, api::endpoint::Endpoint, api::params::QueryParams};

use super::CongressionalLawType;

/// Represents the /law/:congress/:lawType/:lawNumber endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct LawNumber {
    #[builder(setter(into))]
    congress: u16,
    #[builder(default)]
    law_type: CongressionalLawType,
    #[builder(default)]
    law_number: u32,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl LawNumber {
    pub fn builder() -> LawNumberBuilder {
        LawNumberBuilder::default()
    }
}

impl Endpoint for LawNumber {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "law/{}/{}/{}",
            self.congress,
            self.law_type.as_str(),
            self.law_number
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
    use crate::{api::law::law_number::LawNumber, auth::Auth, cdg::Cdg, api::query::Query};

    use super::CongressionalLawType;

    #[test]
    fn is_sufficient() {
        LawNumber::builder()
            .congress(118_u16)
            .law_type(super::CongressionalLawType::Public)
            .law_number(108)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = LawNumber::builder()
            .congress(118_u16)
            .law_type(CongressionalLawType::Public)
            .law_number(108)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
