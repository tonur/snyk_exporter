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
pub struct UpdateOrgRequest {
    #[serde(rename = "data")]
    pub data: Box<crate::models::UpdateOrgRequestData>,
}

impl UpdateOrgRequest {
    pub fn new(data: crate::models::UpdateOrgRequestData) -> UpdateOrgRequest {
        UpdateOrgRequest {
            data: Box::new(data),
        }
    }
}

