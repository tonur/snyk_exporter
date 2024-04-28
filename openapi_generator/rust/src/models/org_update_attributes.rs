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
pub struct OrgUpdateAttributes {
    /// The display name of the organization.
    #[serde(rename = "name")]
    pub name: String,
}

impl OrgUpdateAttributes {
    pub fn new(name: String) -> OrgUpdateAttributes {
        OrgUpdateAttributes {
            name,
        }
    }
}

