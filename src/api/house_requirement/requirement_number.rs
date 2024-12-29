use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{endpoint::Endpoint, params::QueryParams};

use super::Format;

mod matching_communications;

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct RequirementNumber {
    #[builder(setter(into))]
    requirement_number: u32,
    #[builder(default)]
    format: Format,
}

impl RequirementNumber {
    pub fn builder() -> RequirementNumberBuilder {
        RequirementNumberBuilder::default()
    }
}

impl Endpoint for RequirementNumber {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("house-requirement/{}", self.requirement_number).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("format", self.format);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::house_requirement::requirement_number::RequirementNumber, auth::Auth, cdg::Cdg,
        query::Query,
    };

    #[test]
    fn is_sufficient() {
        RequirementNumber::builder()
            .requirement_number(8070_u32)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = RequirementNumber::builder()
            .requirement_number(8070_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
