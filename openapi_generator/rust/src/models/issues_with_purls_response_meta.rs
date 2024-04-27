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
pub struct IssuesWithPurlsResponseMeta {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::GetCustomBaseImages400ResponseErrorsInner>>,
}

impl IssuesWithPurlsResponseMeta {
    pub fn new() -> IssuesWithPurlsResponseMeta {
        IssuesWithPurlsResponseMeta {
            errors: None,
        }
    }
}


