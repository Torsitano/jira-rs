/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// RemoteObjectStatus : The status of the item.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoteObjectStatus {
    /// Whether the item is resolved. If set to \"true\", the link to the issue is displayed in a strikethrough font, otherwise the link displays in normal font.
    #[serde(rename = "resolved", skip_serializing_if = "Option::is_none")]
    pub resolved: Option<bool>,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Box<crate::models::StatusIcon>>,
}

impl RemoteObjectStatus {
    /// The status of the item.
    pub fn new() -> RemoteObjectStatus {
        RemoteObjectStatus {
            resolved: None,
            icon: None,
        }
    }
}
