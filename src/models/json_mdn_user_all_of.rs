/*
 * madana-api
 *
 * <h1>Using the madana-api</h1>        <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available         endpoints and used datamodels.   </p>       <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p>   <br>   <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>       <br>
 *
 * The version of the OpenAPI document: 0.5.0-master.22
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonMdnUserAllOf {
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<crate::models::JsonMdnUserCredentials>,
    /// 
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// 
    #[serde(rename = "socialAccounts", skip_serializing_if = "Option::is_none")]
    pub social_accounts: Option<Vec<crate::models::JsonMdnSocialUserObject>>,
    /// 
    #[serde(rename = "guid", skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    /// 
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// 
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<crate::models::JsonMdnUserSetting>>,
    /// 
    #[serde(rename = "mail", skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
}

impl JsonMdnUserAllOf {
    pub fn new() -> JsonMdnUserAllOf {
        JsonMdnUserAllOf {
            credentials: None,
            last_name: None,
            social_accounts: None,
            guid: None,
            first_name: None,
            settings: None,
            mail: None,
        }
    }
}


