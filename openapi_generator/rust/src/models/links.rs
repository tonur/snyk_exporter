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
pub struct Links {
    #[serde(rename = "first", skip_serializing_if = "Option::is_none")]
    pub first: Option<Box<crate::models::LinkProperty>>,
    #[serde(rename = "last", skip_serializing_if = "Option::is_none")]
    pub last: Option<Box<crate::models::LinkProperty>>,
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<Box<crate::models::LinkProperty>>,
    #[serde(rename = "prev", skip_serializing_if = "Option::is_none")]
    pub prev: Option<Box<crate::models::LinkProperty>>,
    #[serde(rename = "related", skip_serializing_if = "Option::is_none")]
    pub related: Option<Box<crate::models::LinkProperty>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<Box<crate::models::LinkProperty>>,
}

impl Links {
    pub fn new() -> Links {
        Links {
            first: None,
            last: None,
            next: None,
            prev: None,
            related: None,
            param_self: None,
        }
    }
}


