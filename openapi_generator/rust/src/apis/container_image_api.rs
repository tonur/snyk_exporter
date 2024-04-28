/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::{apis::ResponseContent, models::Platform};
use super::{Error, configuration};


/// struct for typed errors of method [`get_container_image`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetContainerImageError {
    Status400(crate::models::GetCustomBaseImages400Response),
    Status401(crate::models::GetCustomBaseImages400Response),
    Status403(crate::models::GetCustomBaseImages400Response),
    Status404(crate::models::GetCustomBaseImages400Response),
    Status409(crate::models::GetCustomBaseImages400Response),
    Status500(crate::models::GetCustomBaseImages400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_container_image`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListContainerImageError {
    Status400(crate::models::GetCustomBaseImages400Response),
    Status401(crate::models::GetCustomBaseImages400Response),
    Status403(crate::models::GetCustomBaseImages400Response),
    Status404(crate::models::GetCustomBaseImages400Response),
    Status409(crate::models::GetCustomBaseImages400Response),
    Status500(crate::models::GetCustomBaseImages400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_image_target_refs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListImageTargetRefsError {
    Status400(crate::models::GetCustomBaseImages400Response),
    Status401(crate::models::GetCustomBaseImages400Response),
    Status403(crate::models::GetCustomBaseImages400Response),
    Status404(crate::models::GetCustomBaseImages400Response),
    Status409(crate::models::GetCustomBaseImages400Response),
    Status500(crate::models::GetCustomBaseImages400Response),
    UnknownValue(serde_json::Value),
}


/// Get instance of container image
pub async fn get_container_image(configuration: &configuration::Configuration, version: &str, org_id: &str, image_id: &str) -> Result<crate::models::GetContainerImage200Response, Error<GetContainerImageError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/orgs/{org_id}/container_images/{image_id}", local_var_configuration.base_path, org_id=crate::apis::urlencode(org_id), image_id=crate::apis::urlencode(image_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("version", &version.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetContainerImageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List instances of container image
pub async fn list_container_image(configuration: &configuration::Configuration, org_id: &str, version: &str, image_ids: Option<Vec<String>>, platform: Option<Platform>, names: Option<Vec<String>>, limit: Option<i32>, starting_after: Option<&str>, ending_before: Option<&str>) -> Result<crate::models::ListContainerImage200Response, Error<ListContainerImageError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/orgs/{org_id}/container_images", local_var_configuration.base_path, org_id=crate::apis::urlencode(org_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = image_ids {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("image_ids".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("image_ids", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = platform {
        local_var_req_builder = local_var_req_builder.query(&[("platform", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = names {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("names".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("names", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    local_var_req_builder = local_var_req_builder.query(&[("version", &version.to_string())]);
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = starting_after {
        local_var_req_builder = local_var_req_builder.query(&[("starting_after", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = ending_before {
        local_var_req_builder = local_var_req_builder.query(&[("ending_before", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListContainerImageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List instances of image target references for a container image
pub async fn list_image_target_refs(configuration: &configuration::Configuration, org_id: &str, image_id: &str, version: &str, limit: Option<i32>, starting_after: Option<&str>, ending_before: Option<&str>) -> Result<crate::models::ListImageTargetRefs200Response, Error<ListImageTargetRefsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/orgs/{org_id}/container_images/{image_id}/relationships/image_target_refs", local_var_configuration.base_path, org_id=crate::apis::urlencode(org_id), image_id=crate::apis::urlencode(image_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("version", &version.to_string())]);
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = starting_after {
        local_var_req_builder = local_var_req_builder.query(&[("starting_after", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = ending_before {
        local_var_req_builder = local_var_req_builder.query(&[("ending_before", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListImageTargetRefsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
