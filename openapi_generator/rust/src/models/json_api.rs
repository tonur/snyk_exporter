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
pub struct JsonApi {
    /// Version of the JSON API specification this server supports.
    #[serde(rename = "version")]
    pub version: String,
}

impl JsonApi {
    pub fn new(version: String) -> JsonApi {
        JsonApi {
            version,
        }
    }
}

