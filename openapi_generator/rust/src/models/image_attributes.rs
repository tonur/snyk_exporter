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
pub struct ImageAttributes {
    #[serde(rename = "layers")]
    pub layers: Vec<String>,
    #[serde(rename = "names", skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "platform")]
    pub platform: crate::models::Platform,
}

impl ImageAttributes {
    pub fn new(layers: Vec<String>, platform: crate::models::Platform) -> ImageAttributes {
        ImageAttributes {
            layers,
            names: None,
            platform,
        }
    }
}


