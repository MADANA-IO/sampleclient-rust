/*
 * madana-api
 *
 * <h1>API Quickstart Guide</h1>        <p>This documentation contains a Quickstart Guide, a few <a href=\"downloads.html\">sample clients</a>  for download and information about the available  <a href=\"resources.html\">endpoints</a>  and  <a href=\"data.html\">DataTypes</a>  </p>     <p>The <a target=\"_blank\" href=\"http://madana-explorer-staging.eu-central-1.elasticbeanstalk.com/login\">  MADANA Explorer</a> can be used to verify the interactions with the API</p>           <p>Internal use only. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a></p>         <br> <br>
 *
 * The version of the OpenAPI document: 0.4.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JsonDatasetInfo : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonDatasetInfo {
    /// 
    #[serde(rename = "fingerpint", skip_serializing_if = "Option::is_none")]
    pub fingerpint: Option<String>,
    /// 
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// 
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// 
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// 
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    /// 
    #[serde(rename = "creationdate", skip_serializing_if = "Option::is_none")]
    pub creationdate: Option<String>,
    /// 
    #[serde(rename = "fingerprint", skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
}

impl JsonDatasetInfo {
    /// 
    pub fn new() -> JsonDatasetInfo {
        JsonDatasetInfo {
            fingerpint: None,
            signature: None,
            data: None,
            size: None,
            hash: None,
            creationdate: None,
            fingerprint: None,
        }
    }
}


