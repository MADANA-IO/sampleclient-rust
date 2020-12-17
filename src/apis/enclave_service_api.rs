/*
 * madana-api
 *
 * <h1>Using the madana-api</h1>        <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available         endpoints and used datamodels.   </p>       <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p>   <br>   <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>       <br>
 *
 * The version of the OpenAPI document: 0.5.0-master.8
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `add_history`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddHistoryError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `approve_enclave`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApproveEnclaveError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `assign_enclave_agent`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssignEnclaveAgentError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `attestate_enclave`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttestateEnclaveError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `create_enclave_run_request`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEnclaveRunRequestError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_enclave`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEnclaveError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_enclave_types`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEnclaveTypesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_enclaves`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEnclavesError {
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `kill_enclave`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KillEnclaveError {
    UnknownValue(serde_json::Value),
}


pub async fn add_history(configuration: &configuration::Configuration, uuid: &str, body: Option<crate::models::JsonSignedData>) -> Result<std::path::PathBuf, Error<AddHistoryError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/enclaves/{uuid}/history", configuration.base_path, uuid=crate::apis::urlencode(uuid));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddHistoryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn approve_enclave(configuration: &configuration::Configuration, uuid: &str, body: Option<crate::models::JsonEnclaveRunningAttestationApproval>) -> Result<std::path::PathBuf, Error<ApproveEnclaveError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/enclaves/{uuid}/approval", configuration.base_path, uuid=crate::apis::urlencode(uuid));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ApproveEnclaveError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn assign_enclave_agent(configuration: &configuration::Configuration, uuid: &str, body: Option<crate::models::JsonNodeInfo>) -> Result<std::path::PathBuf, Error<AssignEnclaveAgentError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/enclaves/{uuid}/assign", configuration.base_path, uuid=crate::apis::urlencode(uuid));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AssignEnclaveAgentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn attestate_enclave(configuration: &configuration::Configuration, uuid: &str, body: Option<crate::models::JsonEnclaveRunningAttestation>) -> Result<std::path::PathBuf, Error<AttestateEnclaveError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/enclaves/{uuid}/attestation", configuration.base_path, uuid=crate::apis::urlencode(uuid));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AttestateEnclaveError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_enclave_run_request(configuration: &configuration::Configuration, body: Option<crate::models::JsonEnclaveRunRequest>) -> Result<std::path::PathBuf, Error<CreateEnclaveRunRequestError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/enclaves", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateEnclaveRunRequestError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_enclave(configuration: &configuration::Configuration, uuid: &str) -> Result<std::path::PathBuf, Error<GetEnclaveError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/enclaves/{uuid}", configuration.base_path, uuid=crate::apis::urlencode(uuid));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetEnclaveError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_enclave_types(configuration: &configuration::Configuration, ) -> Result<std::path::PathBuf, Error<GetEnclaveTypesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/enclaves/types", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetEnclaveTypesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns UUIDs of existing analyses.
pub async fn get_enclaves(configuration: &configuration::Configuration, authorization: Option<&str>, created: Option<&str>, limit: Option<&str>, offset: Option<&str>, status: Option<&str>) -> Result<std::path::PathBuf, Error<GetEnclavesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/enclaves", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = created {
        local_var_req_builder = local_var_req_builder.query(&[("created", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = status {
        local_var_req_builder = local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetEnclavesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn kill_enclave(configuration: &configuration::Configuration, uuid: &str) -> Result<std::path::PathBuf, Error<KillEnclaveError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/enclaves/{uuid}/kill", configuration.base_path, uuid=crate::apis::urlencode(uuid));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<KillEnclaveError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

