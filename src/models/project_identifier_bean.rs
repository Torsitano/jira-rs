/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectIdentifierBean : The identifiers for a project.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectIdentifierBean {
    /// The ID of the project.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The key of the project.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl ProjectIdentifierBean {
    /// The identifiers for a project.
    pub fn new() -> ProjectIdentifierBean {
        ProjectIdentifierBean {
            id: None,
            key: None,
        }
    }
}
