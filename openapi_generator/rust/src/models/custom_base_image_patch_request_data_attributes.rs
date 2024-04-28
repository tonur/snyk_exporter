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
pub struct CustomBaseImagePatchRequestDataAttributes {
    #[serde(rename = "include_in_recommendations", skip_serializing_if = "Option::is_none")]
    pub include_in_recommendations: Option<bool>,
    #[serde(rename = "versioning_schema", skip_serializing_if = "Option::is_none")]
    pub versioning_schema: Option<Box<crate::models::VersioningSchema>>,
}

impl CustomBaseImagePatchRequestDataAttributes {
    pub fn new() -> CustomBaseImagePatchRequestDataAttributes {
        CustomBaseImagePatchRequestDataAttributes {
            include_in_recommendations: None,
            versioning_schema: None,
        }
    }
}

