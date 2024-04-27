/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ErrorLink : A link that leads to further details about this particular occurrance of the problem.



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorLink {
    #[serde(rename = "about", skip_serializing_if = "Option::is_none")]
    pub about: Option<Box<crate::models::GetCustomBaseImages400ResponseErrorsInnerLinksAbout>>,
}

impl ErrorLink {
    /// A link that leads to further details about this particular occurrance of the problem.
    pub fn new() -> ErrorLink {
        ErrorLink {
            about: None,
        }
    }
}


