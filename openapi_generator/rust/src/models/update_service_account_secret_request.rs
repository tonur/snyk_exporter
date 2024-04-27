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
pub struct UpdateServiceAccountSecretRequest {
    #[serde(rename = "data")]
    pub data: Box<crate::models::UpdateServiceAccountSecretRequestData>,
}

impl UpdateServiceAccountSecretRequest {
    pub fn new(data: crate::models::UpdateServiceAccountSecretRequestData) -> UpdateServiceAccountSecretRequest {
        UpdateServiceAccountSecretRequest {
            data: Box::new(data),
        }
    }
}


