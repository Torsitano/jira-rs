/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowSimpleCondition : A workflow transition rule condition. This object returns `nodeType` as `simple`.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowSimpleCondition {
    /// The type of the transition rule.
    #[serde(rename = "type")]
    pub r#type: String,
    /// EXPERIMENTAL. The configuration of the transition rule.
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,
    #[serde(rename = "nodeType")]
    pub node_type: String,
}

impl WorkflowSimpleCondition {
    /// A workflow transition rule condition. This object returns `nodeType` as `simple`.
    pub fn new(r#type: String, node_type: String) -> WorkflowSimpleCondition {
        WorkflowSimpleCondition {
            r#type,
            configuration: None,
            node_type,
        }
    }
}
