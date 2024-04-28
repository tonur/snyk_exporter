/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrganizationType {
    #[serde(rename = "organization")]
    Organization,

}

impl ToString for OrganizationType {
    fn to_string(&self) -> String {
        match self {
            Self::Organization => String::from("organization"),
        }
    }
}

impl Default for OrganizationType {
    fn default() -> OrganizationType {
        Self::Organization
    }
}



