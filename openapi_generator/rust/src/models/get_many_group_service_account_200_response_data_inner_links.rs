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
pub struct GetManyGroupServiceAccount200ResponseDataInnerLinks {
    #[serde(rename = "first", skip_serializing_if = "Option::is_none")]
    pub first: Option<Box<crate::models::GetCustomBaseImages400ResponseErrorsInnerLinksAbout>>,
    #[serde(rename = "last", skip_serializing_if = "Option::is_none")]
    pub last: Option<Box<crate::models::GetCustomBaseImages400ResponseErrorsInnerLinksAbout>>,
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<Box<crate::models::GetCustomBaseImages400ResponseErrorsInnerLinksAbout>>,
    #[serde(rename = "prev", skip_serializing_if = "Option::is_none")]
    pub prev: Option<Box<crate::models::GetCustomBaseImages400ResponseErrorsInnerLinksAbout>>,
    #[serde(rename = "related", skip_serializing_if = "Option::is_none")]
    pub related: Option<Box<crate::models::GetCustomBaseImages400ResponseErrorsInnerLinksAbout>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<Box<crate::models::GetCustomBaseImages400ResponseErrorsInnerLinksAbout>>,
}

impl GetManyGroupServiceAccount200ResponseDataInnerLinks {
    pub fn new() -> GetManyGroupServiceAccount200ResponseDataInnerLinks {
        GetManyGroupServiceAccount200ResponseDataInnerLinks {
            first: None,
            last: None,
            next: None,
            prev: None,
            related: None,
            param_self: None,
        }
    }
}


