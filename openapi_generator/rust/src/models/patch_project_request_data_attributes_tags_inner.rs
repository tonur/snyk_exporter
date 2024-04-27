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
pub struct PatchProjectRequestDataAttributesTagsInner {
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl PatchProjectRequestDataAttributesTagsInner {
    pub fn new() -> PatchProjectRequestDataAttributesTagsInner {
        PatchProjectRequestDataAttributesTagsInner {
            key: None,
            value: None,
        }
    }
}


