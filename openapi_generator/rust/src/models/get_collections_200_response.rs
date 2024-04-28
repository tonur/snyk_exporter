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
pub struct GetCollections200Response {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::CollectionResponse>>,
    #[serde(rename = "jsonapi", skip_serializing_if = "Option::is_none")]
    pub jsonapi: Option<Box<crate::models::JsonApi>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::PaginatedLinks>>,
}

impl GetCollections200Response {
    pub fn new() -> GetCollections200Response {
        GetCollections200Response {
            data: None,
            jsonapi: None,
            links: None,
        }
    }
}

