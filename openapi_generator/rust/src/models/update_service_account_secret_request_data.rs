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
pub struct UpdateServiceAccountSecretRequestData {
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::UpdateServiceAccountSecretRequestDataAttributes>,
    /// The Resource type.
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl UpdateServiceAccountSecretRequestData {
    pub fn new(attributes: crate::models::UpdateServiceAccountSecretRequestDataAttributes, r#type: Type) -> UpdateServiceAccountSecretRequestData {
        UpdateServiceAccountSecretRequestData {
            attributes: Box::new(attributes),
            r#type,
        }
    }
}

/// The Resource type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "service_account")]
    ServiceAccount,
}

impl Default for Type {
    fn default() -> Type {
        Self::ServiceAccount
    }
}
