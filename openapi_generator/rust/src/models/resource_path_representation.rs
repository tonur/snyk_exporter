/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ResourcePathRepresentation : An object that contains an opaque identifying string.



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcePathRepresentation {
    #[serde(rename = "resource_path")]
    pub resource_path: String,
}

impl ResourcePathRepresentation {
    /// An object that contains an opaque identifying string.
    pub fn new(resource_path: String) -> ResourcePathRepresentation {
        ResourcePathRepresentation {
            resource_path,
        }
    }
}

