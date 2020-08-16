/*
 * madana-api
 *
 * <h1>API Quickstart Guide</h1>        <p>This documentation contains a Quickstart Guide, a few <a href=\"downloads.html\">sample clients</a>  for download and information about the available  <a href=\"resources.html\">endpoints</a>  and  <a href=\"data.html\">DataTypes</a>  </p>     <p>The <a target=\"_blank\" href=\"http://madana-explorer-staging.eu-central-1.elasticbeanstalk.com/login\">  MADANA Explorer</a> can be used to verify the interactions with the API</p>           <p>Internal use only. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a></p>         <br> <br>
 *
 * The version of the OpenAPI document: 0.4.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JsonAnalysisResult : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonAnalysisResult {
    /// 
    #[serde(rename = "rawBytes", skip_serializing_if = "Option::is_none")]
    pub raw_bytes: Option<String>,
    /// 
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 
    #[serde(rename = "subGroups", skip_serializing_if = "Option::is_none")]
    pub sub_groups: Option<Vec<crate::models::JsonAnalysisResultSubGroup>>,
    #[serde(rename = "visualization", skip_serializing_if = "Option::is_none")]
    pub visualization: Option<crate::models::JsonAnalysisVisualization>,
}

impl JsonAnalysisResult {
    /// 
    pub fn new() -> JsonAnalysisResult {
        JsonAnalysisResult {
            raw_bytes: None,
            description: None,
            sub_groups: None,
            visualization: None,
        }
    }
}

