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
pub struct IssueRelationshipsIgnoreData {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::IgnoreType,
}

impl IssueRelationshipsIgnoreData {
    pub fn new(id: String, r#type: crate::models::IgnoreType) -> IssueRelationshipsIgnoreData {
        IssueRelationshipsIgnoreData {
            id,
            r#type,
        }
    }
}


