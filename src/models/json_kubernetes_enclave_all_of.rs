/*
 * madana-api
 *
 * <h1>Using the madana-api</h1>        <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available         endpoints and used datamodels.   </p>       <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p>   <br>   <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>       <br>
 *
 * The version of the OpenAPI document: 0.5.0-master.46
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonKubernetesEnclaveAllOf {
    /// 
    #[serde(rename = "wireguardPort", skip_serializing_if = "Option::is_none")]
    pub wireguard_port: Option<i32>,
    /// 
    #[serde(rename = "attestationPort", skip_serializing_if = "Option::is_none")]
    pub attestation_port: Option<i32>,
    /// 
    #[serde(rename = "isUsingInitContainer", skip_serializing_if = "Option::is_none")]
    pub is_using_init_container: Option<bool>,
    /// 
    #[serde(rename = "debugInfo", skip_serializing_if = "Option::is_none")]
    pub debug_info: Option<String>,
    #[serde(rename = "enclaveDeploymentEvents", skip_serializing_if = "Option::is_none")]
    pub enclave_deployment_events: Option<crate::models::JsonV1EventList>,
    /// 
    #[serde(rename = "remoteControlIP", skip_serializing_if = "Option::is_none")]
    pub remote_control_ip: Option<String>,
    #[serde(rename = "enclavePodEvents", skip_serializing_if = "Option::is_none")]
    pub enclave_pod_events: Option<crate::models::JsonV1EventList>,
    /// 
    #[serde(rename = "podPhase", skip_serializing_if = "Option::is_none")]
    pub pod_phase: Option<String>,
    #[serde(rename = "enclaveReplicaSetEvents", skip_serializing_if = "Option::is_none")]
    pub enclave_replica_set_events: Option<crate::models::JsonV1EventList>,
}

impl JsonKubernetesEnclaveAllOf {
    pub fn new() -> JsonKubernetesEnclaveAllOf {
        JsonKubernetesEnclaveAllOf {
            wireguard_port: None,
            attestation_port: None,
            is_using_init_container: None,
            debug_info: None,
            enclave_deployment_events: None,
            remote_control_ip: None,
            enclave_pod_events: None,
            pod_phase: None,
            enclave_replica_set_events: None,
        }
    }
}


