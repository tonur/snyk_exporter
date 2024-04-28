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
pub struct CloudResource {
    #[serde(rename = "environment")]
    pub environment: Box<crate::models::Environment>,
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<Box<crate::models::Resource>>,
}

impl CloudResource {
    pub fn new(environment: crate::models::Environment) -> CloudResource {
        CloudResource {
            environment: Box::new(environment),
            resource: None,
        }
    }
}

