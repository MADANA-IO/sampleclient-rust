/*
 * madana-api
 *
 * <h1>API Quickstart Guide</h1>        <p>This documentation contains a Quickstart Guide, a few <a href=\"downloads.html\">sample clients</a>  for download and information about the available  <a href=\"resources.html\">endpoints</a>  and  <a href=\"data.html\">DataTypes</a>  </p>     <p>The <a target=\"_blank\" href=\"http://madana-explorer-staging.eu-central-1.elasticbeanstalk.com/login\">  MADANA Explorer</a> can be used to verify the interactions with the API</p>           <p>Internal use only. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a></p>         <br> <br>
 *
 * The version of the OpenAPI document: 0.4.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JsonAnalysis : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonAnalysis {
    /// 
    #[serde(rename = "tokenAmount", skip_serializing_if = "Option::is_none")]
    pub token_amount: Option<String>,
    /// 
    #[serde(rename = "treshold", skip_serializing_if = "Option::is_none")]
    pub treshold: Option<String>,
    /// 
    #[serde(rename = "dataCollectionMethod", skip_serializing_if = "Option::is_none")]
    pub data_collection_method: Option<String>,
    /// 
    #[serde(rename = "dataCollectionConfig", skip_serializing_if = "Option::is_none")]
    pub data_collection_config: Option<String>,
    /// 
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// 
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// 
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 
    #[serde(rename = "agent", skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// 
    #[serde(rename = "datasets", skip_serializing_if = "Option::is_none")]
    pub datasets: Option<Vec<crate::models::JsonDatasetInfo>>,
    /// 
    #[serde(rename = "participants", skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<String>>,
    /// 
    #[serde(rename = "views", skip_serializing_if = "Option::is_none")]
    pub views: Option<String>,
    /// 
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<crate::models::JsonAnalysisRequestAction>>,
    /// 
    #[serde(rename = "creator", skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 
    #[serde(rename = "datasetCount", skip_serializing_if = "Option::is_none")]
    pub dataset_count: Option<String>,
}

impl JsonAnalysis {
    /// 
    pub fn new() -> JsonAnalysis {
        JsonAnalysis {
            token_amount: None,
            treshold: None,
            data_collection_method: None,
            data_collection_config: None,
            description: None,
            created: None,
            uuid: None,
            status: None,
            agent: None,
            datasets: None,
            participants: None,
            views: None,
            actions: None,
            creator: None,
            dataset_count: None,
        }
    }
}


