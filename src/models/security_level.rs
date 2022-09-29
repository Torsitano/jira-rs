/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// SecurityLevel : Details of an issue level security item.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityLevel {
    /// The URL of the issue level security item.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The ID of the issue level security item.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The description of the issue level security item.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the issue level security item.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SecurityLevel {
    /// Details of an issue level security item.
    pub fn new() -> SecurityLevel {
        SecurityLevel {
            param_self: None,
            id: None,
            description: None,
            name: None,
        }
    }
}
