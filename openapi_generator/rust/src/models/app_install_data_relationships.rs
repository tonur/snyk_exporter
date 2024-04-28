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
pub struct AppInstallDataRelationships {
    #[serde(rename = "app", skip_serializing_if = "Option::is_none")]
    pub app: Option<Box<crate::models::AppInstallDataRelationshipsApp>>,
}

impl AppInstallDataRelationships {
    pub fn new() -> AppInstallDataRelationships {
        AppInstallDataRelationships {
            app: None,
        }
    }
}

