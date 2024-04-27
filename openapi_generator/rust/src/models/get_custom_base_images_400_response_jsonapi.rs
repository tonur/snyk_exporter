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
pub struct GetCustomBaseImages400ResponseJsonapi {
    /// Version of the JSON API specification this server supports.
    #[serde(rename = "version")]
    pub version: String,
}

impl GetCustomBaseImages400ResponseJsonapi {
    pub fn new(version: String) -> GetCustomBaseImages400ResponseJsonapi {
        GetCustomBaseImages400ResponseJsonapi {
            version,
        }
    }
}


