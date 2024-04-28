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
pub struct ListOrgProjects200ResponseDataInner {
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::ProjectAttributes>,
    /// Resource ID.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListOrgProjects200ResponseDataInnerMeta>>,
    // #TODO - Relationships cause a StackOverflow error, need to investigate what causes this.
    // #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    // pub relationships: Option<Box<crate::models::ProjectRelationships>>,
    /// The Resource type.
    #[serde(rename = "type")]
    pub r#type: String,
}

impl ListOrgProjects200ResponseDataInner {
    pub fn new(attributes: crate::models::ProjectAttributes, id: uuid::Uuid, r#type: String) -> ListOrgProjects200ResponseDataInner {
        ListOrgProjects200ResponseDataInner {
            attributes: Box::new(attributes),
            id,
            meta: None,
            // relationships: None,
            r#type,
        }
    }
}


