/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OrgIacSettingsResponse : The Infrastructure as Code settings for an org.



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrgIacSettingsResponse {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::OrgIacSettingsResponseAttributes>>,
    /// ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// Content type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl OrgIacSettingsResponse {
    /// The Infrastructure as Code settings for an org.
    pub fn new() -> OrgIacSettingsResponse {
        OrgIacSettingsResponse {
            attributes: None,
            id: None,
            r#type: None,
        }
    }
}


