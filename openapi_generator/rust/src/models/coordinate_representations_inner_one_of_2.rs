/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CoordinateRepresentationsInnerOneOf2 : A resource location to some service, like a cloud resource.



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoordinateRepresentationsInnerOneOf2 {
    #[serde(rename = "cloud_resource")]
    pub cloud_resource: Box<crate::models::CloudResource>,
}

impl CoordinateRepresentationsInnerOneOf2 {
    /// A resource location to some service, like a cloud resource.
    pub fn new(cloud_resource: crate::models::CloudResource) -> CoordinateRepresentationsInnerOneOf2 {
        CoordinateRepresentationsInnerOneOf2 {
            cloud_resource: Box::new(cloud_resource),
        }
    }
}

