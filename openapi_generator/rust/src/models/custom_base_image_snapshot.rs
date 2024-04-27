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
pub struct CustomBaseImageSnapshot {
    #[serde(rename = "group_id", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<uuid::Uuid>,
    #[serde(rename = "include_in_recommendations", skip_serializing_if = "Option::is_none")]
    pub include_in_recommendations: Option<bool>,
    #[serde(rename = "org_id", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<uuid::Uuid>,
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<uuid::Uuid>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl CustomBaseImageSnapshot {
    pub fn new() -> CustomBaseImageSnapshot {
        CustomBaseImageSnapshot {
            group_id: None,
            include_in_recommendations: None,
            org_id: None,
            project_id: None,
            repository: None,
            tag: None,
        }
    }
}


