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
pub struct CoordinateVTwoRepresentationsInner {
    #[serde(rename = "resource_path")]
    pub resource_path: String,
    #[serde(rename = "package", skip_serializing_if = "Option::is_none")]
    pub package: Option<Box<crate::models::PackageMeta>>,
}

impl CoordinateVTwoRepresentationsInner {
    pub fn new(resource_path: String) -> CoordinateVTwoRepresentationsInner {
        CoordinateVTwoRepresentationsInner {
            resource_path,
            package: None,
        }
    }
}


