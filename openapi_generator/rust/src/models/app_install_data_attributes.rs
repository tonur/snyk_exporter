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
pub struct AppInstallDataAttributes {
    /// The oauth2 client id for the app.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<uuid::Uuid>,
}

impl AppInstallDataAttributes {
    pub fn new() -> AppInstallDataAttributes {
        AppInstallDataAttributes {
            client_id: None,
        }
    }
}

