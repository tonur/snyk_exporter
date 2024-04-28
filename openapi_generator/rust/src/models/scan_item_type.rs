/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ScanItemType {
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "environment")]
    Environment,

}

impl ToString for ScanItemType {
    fn to_string(&self) -> String {
        match self {
            Self::Project => String::from("project"),
            Self::Environment => String::from("environment"),
        }
    }
}

impl Default for ScanItemType {
    fn default() -> ScanItemType {
        Self::Project
    }
}



