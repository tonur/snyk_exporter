/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */

use super::DeployedRiskFactor;
use super::OsConditionRiskFactor;
use super::PublicFacingRiskFactor;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum RiskFactor {
    DeployedRiskFactor(Box<DeployedRiskFactor>),
    OsConditionRiskFactor(Box<OsConditionRiskFactor>),
    PublicFacingRiskFactor(Box<PublicFacingRiskFactor>),
}

impl Default for RiskFactor {
    fn default() -> Self {
        Self::DeployedRiskFactor(Box::default())
    }
}




