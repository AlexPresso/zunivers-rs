use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pack {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub year: Option<u16>
}

