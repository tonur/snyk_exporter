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
pub struct CreateGroupAppInstall201Response {
    #[serde(rename = "data")]
    pub data: Box<crate::models::AppInstallWithClient>,
    #[serde(rename = "jsonapi")]
    pub jsonapi: Box<crate::models::JsonApi>,
    #[serde(rename = "links")]
    pub links: Box<crate::models::PaginatedLinks>,
}

impl CreateGroupAppInstall201Response {
    pub fn new(data: crate::models::AppInstallWithClient, jsonapi: crate::models::JsonApi, links: crate::models::PaginatedLinks) -> CreateGroupAppInstall201Response {
        CreateGroupAppInstall201Response {
            data: Box::new(data),
            jsonapi: Box::new(jsonapi),
            links: Box::new(links),
        }
    }
}

