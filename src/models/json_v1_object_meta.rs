/*
 * madana-api
 *
 * <h1>Using the madana-api</h1>        <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available         endpoints and used datamodels.   </p>       <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p>   <br>   <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>       <br>
 *
 * The version of the OpenAPI document: 0.5.0-master.22
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JsonV1ObjectMeta : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonV1ObjectMeta {
    /// 
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f32>,
    /// 
    #[serde(rename = "resourceVersion", skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,
    /// 
    #[serde(rename = "generation", skip_serializing_if = "Option::is_none")]
    pub generation: Option<f32>,
    /// 
    #[serde(rename = "deletionTimestamp", skip_serializing_if = "Option::is_none")]
    pub deletion_timestamp: Option<f32>,
    /// 
    #[serde(rename = "clusterName", skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// 
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// 
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// 
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// 
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<::std::collections::HashMap<String, String>>,
    /// 
    #[serde(rename = "generateName", skip_serializing_if = "Option::is_none")]
    pub generate_name: Option<String>,
    /// 
    #[serde(rename = "deletionGracePeriodSeconds", skip_serializing_if = "Option::is_none")]
    pub deletion_grace_period_seconds: Option<f32>,
    /// 
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// 
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 
    #[serde(rename = "managedFields", skip_serializing_if = "Option::is_none")]
    pub managed_fields: Option<Vec<crate::models::JsonV1ManagedFieldsEntry>>,
    /// 
    #[serde(rename = "ownerReferences", skip_serializing_if = "Option::is_none")]
    pub owner_references: Option<Vec<crate::models::JsonV1OwnerReference>>,
    /// 
    #[serde(rename = "finalizers", skip_serializing_if = "Option::is_none")]
    pub finalizers: Option<Vec<String>>,
}

impl JsonV1ObjectMeta {
    /// 
    pub fn new() -> JsonV1ObjectMeta {
        JsonV1ObjectMeta {
            creation_timestamp: None,
            resource_version: None,
            generation: None,
            deletion_timestamp: None,
            cluster_name: None,
            labels: None,
            namespace: None,
            self_link: None,
            annotations: None,
            generate_name: None,
            deletion_grace_period_seconds: None,
            uid: None,
            name: None,
            managed_fields: None,
            owner_references: None,
            finalizers: None,
        }
    }
}


