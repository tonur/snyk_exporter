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
pub struct UpdateGroupAppInstallSecretRequestDataAttributes {
    /// Operation to perform:   * `replace` - Replace existing secrets with a new generated secret   * `create` - Add a new secret, preserving existing secrets   * `delete` - Remove an existing secret by value 
    #[serde(rename = "mode")]
    pub mode: Mode,
    /// Secret to delete when using `delete` mode
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl UpdateGroupAppInstallSecretRequestDataAttributes {
    pub fn new(mode: Mode) -> UpdateGroupAppInstallSecretRequestDataAttributes {
        UpdateGroupAppInstallSecretRequestDataAttributes {
            mode,
            secret: None,
        }
    }
}

/// Operation to perform:   * `replace` - Replace existing secrets with a new generated secret   * `create` - Add a new secret, preserving existing secrets   * `delete` - Remove an existing secret by value 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "delete")]
    Delete,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::Replace
    }
}
