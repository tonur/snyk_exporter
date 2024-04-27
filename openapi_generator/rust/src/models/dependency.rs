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
pub struct Dependency {
    /// The package name the issue was found in
    #[serde(rename = "package_name")]
    pub package_name: String,
    /// The package version the issue was found in
    #[serde(rename = "package_version")]
    pub package_version: String,
}

impl Dependency {
    pub fn new(package_name: String, package_version: String) -> Dependency {
        Dependency {
            package_name,
            package_version,
        }
    }
}


