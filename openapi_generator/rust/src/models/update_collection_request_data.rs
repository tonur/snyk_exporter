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
pub struct UpdateCollectionRequestData {
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::CreateCollectionRequestDataAttributes>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl UpdateCollectionRequestData {
    pub fn new(attributes: crate::models::CreateCollectionRequestDataAttributes, r#type: String) -> UpdateCollectionRequestData {
        UpdateCollectionRequestData {
            attributes: Box::new(attributes),
            id: None,
            r#type,
        }
    }
}


