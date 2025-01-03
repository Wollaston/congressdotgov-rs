use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{
    api::{CommitteeChamber, Format},
    api::endpoint::Endpoint,
    api::params::QueryParams,
};

/// Represents the /committee-print/:congress/:chamber/:jacketNumber/text endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Text {
    #[builder(setter(into))]
    congress: u16,
    #[builder(setter(into))]
    chamber: CommitteeChamber,
    #[builder(setter(into))]
    jacket_number: u32,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl Text {
    pub fn builder() -> TextBuilder {
        TextBuilder::default()
    }
}

impl Endpoint for Text {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "committee-print/{}/{}/{}/text",
            self.congress,
            self.chamber.as_str(),
            self.jacket_number
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
        Text::builder()
            .congress(117_u16)
            .chamber(CommitteeChamber::House)
            .jacket_number(48144_u32)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = Text::builder()
            .congress(117_u16)
            .chamber(CommitteeChamber::House)
            .jacket_number(48144_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}