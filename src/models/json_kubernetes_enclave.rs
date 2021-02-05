/*
 * madana-api
 *
 * <h1>Using the madana-api</h1>        <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available         endpoints and used datamodels.   </p>       <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p>   <br>   <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>       <br>
 *
 * The version of the OpenAPI document: 0.5.0-master.48
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JsonKubernetesEnclave : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonKubernetesEnclave {
    /// 
    #[serde(rename = "internalRemoteControlServer", skip_serializing_if = "Option::is_none")]
    pub internal_remote_control_server: Option<String>,
    /// 
    #[serde(rename = "wireguardServer", skip_serializing_if = "Option::is_none")]
    pub wireguard_server: Option<String>,
    /// 
    #[serde(rename = "endingTime", skip_serializing_if = "Option::is_none")]
    pub ending_time: Option<String>,
    /// 
    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<crate::models::JsonEnclavePort>>,
    /// 
    #[serde(rename = "publicIdent", skip_serializing_if = "Option::is_none")]
    pub public_ident: Option<String>,
    /// 
    #[serde(rename = "internalWireguardServer", skip_serializing_if = "Option::is_none")]
    pub internal_wireguard_server: Option<String>,
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<crate::models::JsonEnvironment>,
    /// 
    #[serde(rename = "startupCMD", skip_serializing_if = "Option::is_none")]
    pub startup_cmd: Option<String>,
    /// 
    #[serde(rename = "internalAttesationServer", skip_serializing_if = "Option::is_none")]
    pub internal_attesation_server: Option<String>,
    /// 
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 
    #[serde(rename = "remoteControlServer", skip_serializing_if = "Option::is_none")]
    pub remote_control_server: Option<String>,
    /// 
    #[serde(rename = "startupTime", skip_serializing_if = "Option::is_none")]
    pub startup_time: Option<String>,
    /// 
    #[serde(rename = "enclaveInputstream", skip_serializing_if = "Option::is_none")]
    pub enclave_inputstream: Option<serde_json::Value>,
    /// 
    #[serde(rename = "wireguardPublicKey", skip_serializing_if = "Option::is_none")]
    pub wireguard_public_key: Option<String>,
    /// 
    #[serde(rename = "internalIdent", skip_serializing_if = "Option::is_none")]
    pub internal_ident: Option<String>,
    #[serde(rename = "process", skip_serializing_if = "Option::is_none")]
    pub process: Option<crate::models::JsonProcess>,
    /// 
    #[serde(rename = "attestationServer", skip_serializing_if = "Option::is_none")]
    pub attestation_server: Option<String>,
    /// 
    #[serde(rename = "signerIdent", skip_serializing_if = "Option::is_none")]
    pub signer_ident: Option<String>,
    /// 
    #[serde(rename = "consoleOutput", skip_serializing_if = "Option::is_none")]
    pub console_output: Option<String>,
    #[serde(rename = "wgInterface", skip_serializing_if = "Option::is_none")]
    pub wg_interface: Option<crate::models::JsonWireguardInterface>,
    /// 
    #[serde(rename = "enclaveIdent", skip_serializing_if = "Option::is_none")]
    pub enclave_ident: Option<String>,
    #[serde(rename = "kubernetesEnclave", skip_serializing_if = "Option::is_none")]
    pub kubernetes_enclave: Option<crate::models::JsonKubernetesEnclave>,
    /// 
    #[serde(rename = "portMapping", skip_serializing_if = "Option::is_none")]
    pub port_mapping: Option<::std::collections::HashMap<String, String>>,
    /// 
    #[serde(rename = "attestationPort", skip_serializing_if = "Option::is_none")]
    pub attestation_port: Option<i32>,
    /// 
    #[serde(rename = "isUsingInitContainer", skip_serializing_if = "Option::is_none")]
    pub is_using_init_container: Option<bool>,
    /// 
    #[serde(rename = "wireguardPort", skip_serializing_if = "Option::is_none")]
    pub wireguard_port: Option<i32>,
    #[serde(rename = "enclaveDeploymentEvents", skip_serializing_if = "Option::is_none")]
    pub enclave_deployment_events: Option<crate::models::JsonV1EventList>,
    #[serde(rename = "enclaveReplicaSetEvents", skip_serializing_if = "Option::is_none")]
    pub enclave_replica_set_events: Option<crate::models::JsonV1EventList>,
    /// 
    #[serde(rename = "podPhase", skip_serializing_if = "Option::is_none")]
    pub pod_phase: Option<String>,
    /// 
    #[serde(rename = "remoteControlIP", skip_serializing_if = "Option::is_none")]
    pub remote_control_ip: Option<String>,
    #[serde(rename = "enclavePodEvents", skip_serializing_if = "Option::is_none")]
    pub enclave_pod_events: Option<crate::models::JsonV1EventList>,
    /// 
    #[serde(rename = "debugInfo", skip_serializing_if = "Option::is_none")]
    pub debug_info: Option<String>,
}

impl JsonKubernetesEnclave {
    /// 
    pub fn new() -> JsonKubernetesEnclave {
        JsonKubernetesEnclave {
            internal_remote_control_server: None,
            wireguard_server: None,
            ending_time: None,
            ports: None,
            public_ident: None,
            internal_wireguard_server: None,
            environment: None,
            startup_cmd: None,
            internal_attesation_server: None,
            status: None,
            remote_control_server: None,
            startup_time: None,
            enclave_inputstream: None,
            wireguard_public_key: None,
            internal_ident: None,
            process: None,
            attestation_server: None,
            signer_ident: None,
            console_output: None,
            wg_interface: None,
            enclave_ident: None,
            kubernetes_enclave: None,
            port_mapping: None,
            attestation_port: None,
            is_using_init_container: None,
            wireguard_port: None,
            enclave_deployment_events: None,
            enclave_replica_set_events: None,
            pod_phase: None,
            remote_control_ip: None,
            enclave_pod_events: None,
            debug_info: None,
        }
    }
}


