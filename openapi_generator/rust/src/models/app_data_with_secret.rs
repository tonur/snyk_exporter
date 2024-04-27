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
pub struct AppDataWithSecret {
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::AppResourceAttributesWithSecret>,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl AppDataWithSecret {
    pub fn new(attributes: crate::models::AppResourceAttributesWithSecret, id: uuid::Uuid, r#type: String) -> AppDataWithSecret {
        AppDataWithSecret {
            attributes: Box::new(attributes),
            id,
            r#type,
        }
    }
}


