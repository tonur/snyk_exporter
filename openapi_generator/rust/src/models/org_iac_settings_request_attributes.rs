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
pub struct OrgIacSettingsRequestAttributes {
    #[serde(rename = "custom_rules", skip_serializing_if = "Option::is_none")]
    pub custom_rules: Option<Box<crate::models::OrgIacSettingsRequestAttributesCustomRules>>,
}

impl OrgIacSettingsRequestAttributes {
    pub fn new() -> OrgIacSettingsRequestAttributes {
        OrgIacSettingsRequestAttributes {
            custom_rules: None,
        }
    }
}

