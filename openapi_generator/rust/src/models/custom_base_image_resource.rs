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
pub struct CustomBaseImageResource {
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::CustomBaseImageAttributes>,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CustomBaseImageResource {
    pub fn new(attributes: crate::models::CustomBaseImageAttributes, id: uuid::Uuid, r#type: String) -> CustomBaseImageResource {
        CustomBaseImageResource {
            attributes: Box::new(attributes),
            id,
            r#type,
        }
    }
}


