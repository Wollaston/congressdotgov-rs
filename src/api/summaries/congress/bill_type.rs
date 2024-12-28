use chrono::{DateTime, Utc};
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{endpoint::Endpoint, params::QueryParams};

use super::{Format, Sort};

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct BillType {
    #[builder(setter(into))]
    congress: u16,
    #[builder(setter(into))]
    bill_type: crate::api::BillType,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
    #[builder(default)]
    from_date_time: Option<DateTime<Utc>>,
    #[builder(default)]
    to_date_time: Option<DateTime<Utc>>,
    #[builder(default)]
    sort: Option<Sort>,
}

impl BillType {
    pub fn builder() -> BillTypeBuilder {
        BillTypeBuilder::default()
    }
}

impl Endpoint for BillType {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("summaries/{}/{}", self.congress, self.bill_type.as_str()).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("format", self.format);
        params.push_opt("offset", self.offset);
        params.push_opt("limit", self.limit);
        params.push_opt("from_date_time", self.from_date_time);
        params.push_opt("to_date_time", self.to_date_time);
        params.push_opt("sort", self.sort);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::summaries::congress::bill_type::BillType, auth::Auth, cdg::Cdg, query::Query,
    };

    #[test]
    fn is_sufficient() {
        BillType::builder()
            .congress(118_u16)
            .bill_type(crate::api::BillType::Hr)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = BillType::builder()
            .congress(118_u16)
            .bill_type(crate::api::BillType::Hr)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}