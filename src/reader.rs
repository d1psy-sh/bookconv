use std::error::Error;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bookmarks {
    pub checksum: String,
    pub roots: Roots,
    pub version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Roots {
    #[serde(rename = "bookmark_bar")]
    pub bookmark_bar: BookmarkBar,
    pub other: Other,
    pub synced: Synced,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkBar {
    pub children: Vec<Children>,
    #[serde(rename = "date_added")]
    pub date_added: String,
    #[serde(rename = "date_last_used")]
    pub date_last_used: String,
    #[serde(rename = "date_modified")]
    pub date_modified: String,
    pub guid: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children {
    #[serde(rename = "date_added")]
    pub date_added: String,
    #[serde(rename = "date_last_used")]
    pub date_last_used: String,
    pub guid: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: Option<String>,
    #[serde(rename = "meta_info")]
    pub meta_info: Option<MetaInfo>,
    #[serde(default)]
    pub children: Vec<Children2>,
    #[serde(rename = "date_modified")]
    pub date_modified: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaInfo {
    #[serde(rename = "power_bookmark_meta")]
    pub power_bookmark_meta: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children2 {
    #[serde(rename = "date_added")]
    pub date_added: String,
    #[serde(rename = "date_last_used")]
    pub date_last_used: String,
    pub guid: String,
    pub id: String,
    #[serde(rename = "meta_info")]
    pub meta_info: Option<MetaInfo2>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: Option<String>,
    #[serde(default)]
    pub children: Vec<Children3>,
    #[serde(rename = "date_modified")]
    pub date_modified: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaInfo2 {
    #[serde(rename = "power_bookmark_meta")]
    pub power_bookmark_meta: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children3 {
    #[serde(rename = "date_added")]
    pub date_added: String,
    #[serde(rename = "date_last_used")]
    pub date_last_used: String,
    pub guid: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    #[serde(rename = "meta_info")]
    pub meta_info: Option<MetaInfo3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaInfo3 {
    #[serde(rename = "power_bookmark_meta")]
    pub power_bookmark_meta: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Other {
    pub children: Vec<Value>,
    #[serde(rename = "date_added")]
    pub date_added: String,
    #[serde(rename = "date_last_used")]
    pub date_last_used: String,
    #[serde(rename = "date_modified")]
    pub date_modified: String,
    pub guid: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Synced {
    pub children: Vec<Value>,
    #[serde(rename = "date_added")]
    pub date_added: String,
    #[serde(rename = "date_last_used")]
    pub date_last_used: String,
    #[serde(rename = "date_modified")]
    pub date_modified: String,
    pub guid: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

pub fn read_file(input: String) -> Result<Bookmarks, Box<dyn Error>> {
    let bookmark: Bookmarks = serde_json::from_str(&input)?;
    Ok(bookmark)
}

