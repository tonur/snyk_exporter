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
pub struct UpdateOrgSastSettingsRequest {
    #[serde(rename = "data")]
    pub data: Box<crate::models::UpdateOrgSastSettingsRequestData>,
}

impl UpdateOrgSastSettingsRequest {
    pub fn new(data: crate::models::UpdateOrgSastSettingsRequestData) -> UpdateOrgSastSettingsRequest {
        UpdateOrgSastSettingsRequest {
            data: Box::new(data),
        }
    }
}

