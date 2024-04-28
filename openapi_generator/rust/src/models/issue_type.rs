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
pub enum IssueType {
    #[serde(rename = "issue")]
    Issue,

}

impl ToString for IssueType {
    fn to_string(&self) -> String {
        match self {
            Self::Issue => String::from("issue"),
        }
    }
}

impl Default for IssueType {
    fn default() -> IssueType {
        Self::Issue
    }
}



