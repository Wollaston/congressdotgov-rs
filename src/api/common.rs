//! API types common to many endpoints.

use std::borrow::Cow;

/// Chamber options for Committee endpoints.
///
/// This differs from the Chamber enum for the Committee
/// resource as this has the NoChamber variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitteeChamber {
    House,
    Senate,
    NoChamber,
}

impl CommitteeChamber {
    pub fn as_str(self) -> &'static str {
        match self {
            CommitteeChamber::House => "house",
            CommitteeChamber::Senate => "senate",
            CommitteeChamber::NoChamber => "nochamber",
        }
    }
}

/// The possible Congressional bill types for both
/// the House of Representatives and Senate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillType {
    /// H.R. - House Bill
    Hr,
    /// S. - Senate Bill
    S,
    /// H.J. Res. - House Joint Resolution
    Hjres,
    /// S.J. Res. - Senate Joint Resolution
    Sjres,
    /// S.J. Res. - House Concurrent Resolution
    Hconres,
    /// S. Con. Res. - Senate Concurrent Resolution
    Sconres,
    /// H. Res. - House Simple Resolution
    Hres,
    /// S. Res - Senate Simple Resolution
    Sres,
}

impl BillType {
    pub fn as_str(self) -> &'static str {
        match self {
            BillType::Hr => "hr",
            BillType::S => "s",
            BillType::Hjres => "hjres",
            BillType::Sjres => "sjres",
            BillType::Hconres => "hconres",
            BillType::Sconres => "sconres",
            BillType::Hres => "hres",
            BillType::Sres => "sres",
        }
    }
}

/// The congress.gov API can return data in either Json or XML
/// format. The default for this crate is Json.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Json,
    Xml,
}

impl Default for Format {
    fn default() -> Self {
        Self::Json
    }
}

impl From<Format> for Cow<'static, str> {
    fn from(format: Format) -> Self {
        format.as_str().into()
    }
}

impl Format {
    pub fn as_str(self) -> &'static str {
        match self {
            Format::Json => "json",
            Format::Xml => "xml",
        }
    }
}

/// Certain endpoints allow the response to be sorted
/// in either Ascending or Descending order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sort {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

impl Default for Sort {
    fn default() -> Self {
        Self::Asc
    }
}

impl Sort {
    pub fn as_str(self) -> &'static str {
        match self {
            Sort::Asc => "asc",
            Sort::Desc => "desc",
        }
    }
}