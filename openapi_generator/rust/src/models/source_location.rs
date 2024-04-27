/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceLocation {
    /// A path to the file containing this issue, relative to the root of the project target, formatted using POSIX separators. 
    #[serde(rename = "file")]
    pub file: String,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<Box<crate::models::Region>>,
}

impl SourceLocation {
    pub fn new(file: String) -> SourceLocation {
        SourceLocation {
            file,
            region: None,
        }
    }
}


