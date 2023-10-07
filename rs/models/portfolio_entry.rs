use serde::{Serialize,Deserialize};
use sqlx::types::chrono::DateTime;

#[derive(Debug, Serialize,Deserialize, sqlx::FromRow)]
pub struct PortfolioEntry{
    pub entry_id:   i64, 
    pub version: Option<i8>, 
    pub visible: Option<i8>,
    pub name: String,
    pub body: Option<String>,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct PortfolioListing (pub Vec<PortfolioEntry>);

pub enum PortfolioEntryVersion{
    None,
    Legacy,
    V1,
    Invalid, 
}

impl From<Option<i8>> for PortfolioEntryVersion {
    fn from(value: Option<i8>) -> Self {
        if let Some(ver) = value {
            ver.into()
        } else {
            Self::None
        }
    }
}
impl From<i8> for PortfolioEntryVersion {
    fn from(value: i8) -> Self {
           match value {
                0 => Self::None,
                1 => Self::Legacy,
                2 => Self::V1,
                _ => Self::Invalid,
           } 
    } 
}
impl From<PortfolioEntryVersion> for i8 {
    fn from(value: PortfolioEntryVersion) -> Self {
           match value {
                PortfolioEntryVersion::None => 0,
                PortfolioEntryVersion::Legacy => 1 ,
                PortfolioEntryVersion::V1 => 2,
                PortfolioEntryVersion::Invalid => -127,
           } 
    }
} 

impl PortfolioEntry {
    pub async fn get_all_visible(db: &sqlx::MySqlPool, min_version: PortfolioEntryVersion) -> PortfolioListing{
       let ver: i8 = min_version.into();
       PortfolioListing(sqlx::query_as!(PortfolioEntry, 
                                                r#"SELECT * from portfolio_entries WHERE visible=1 AND version >= ? ORDER BY created_at DESC;"#, ver)

                        .fetch_all(db)
                        .await
                        .unwrap())         
    }

    pub async fn get_all_legacy(db: &sqlx::MySqlPool) -> PortfolioListing 
    {
        let ver: i8 = PortfolioEntryVersion::Legacy.into();
        PortfolioListing(sqlx::query_as!(PortfolioEntry,
                                                r#"SELECT * from portfolio_entries WHERE visible=1 AND version = ? ORDER BY entry_id DESC;"#, ver)
                         .fetch_all(db)
                         .await
                         .unwrap())
    }
}

