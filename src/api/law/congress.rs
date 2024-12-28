use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::Format, endpoint::Endpoint, params::QueryParams};

mod law_type;

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Summaries {
    #[builder(setter(into))]
    congress: u16,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl Summaries {
    pub fn builder() -> SummariesBuilder {
        SummariesBuilder::default()
    }
}

impl Endpoint for Summaries {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("law/{}", self.congress).into()
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
    use crate::{api::law::congress::Summaries, auth::Auth, cdg::Cdg, query::Query};

    #[test]
    fn is_sufficient() {
        Summaries::builder().congress(118_u16).build().unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = Summaries::builder().congress(118_u16).build().unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}