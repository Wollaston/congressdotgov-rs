use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{endpoint::Endpoint, params::QueryParams};

use super::Format;

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct SenateCommunication<'a> {
    #[builder(setter(into))]
    chamber: crate::api::Chamber,
    #[builder(setter(into))]
    committee_code: Cow<'a, str>,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl<'a> SenateCommunication<'a> {
    pub fn builder() -> SenateCommunicationBuilder<'a> {
        SenateCommunicationBuilder::default()
    }
}

impl Endpoint for SenateCommunication<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "committee/{}/{}/senate-communication",
            self.chamber.as_str(),
            self.committee_code
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
        api::committee::chamber::committee_code::senate_communications::SenateCommunication,
        auth::Auth, cdg::Cdg, query::Query,
    };

    #[test]
    fn is_sufficient() {
        SenateCommunication::builder()
            .chamber(crate::api::Chamber::Senate)
            .committee_code("hspw00")
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = SenateCommunication::builder()
            .chamber(crate::api::Chamber::Senate)
            .committee_code("ssas00")
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
