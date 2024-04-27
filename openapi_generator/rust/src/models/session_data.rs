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
pub struct SessionData {
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::SessionAttributes>,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl SessionData {
    pub fn new(attributes: crate::models::SessionAttributes, id: uuid::Uuid, r#type: String) -> SessionData {
        SessionData {
            attributes: Box::new(attributes),
            id,
            r#type,
        }
    }
}


