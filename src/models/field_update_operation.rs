/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// FieldUpdateOperation : Details of an operation to perform on a field.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FieldUpdateOperation {
    /// The value to add to the field.
    #[serde(rename = "add", skip_serializing_if = "Option::is_none")]
    pub add: Option<serde_json::Value>,
    /// The value to set in the field.
    #[serde(rename = "set", skip_serializing_if = "Option::is_none")]
    pub set: Option<serde_json::Value>,
    /// The value to removed from the field.
    #[serde(rename = "remove", skip_serializing_if = "Option::is_none")]
    pub remove: Option<serde_json::Value>,
    /// The value to edit in the field.
    #[serde(rename = "edit", skip_serializing_if = "Option::is_none")]
    pub edit: Option<serde_json::Value>,
    /// The field value to copy from another issue.
    #[serde(rename = "copy", skip_serializing_if = "Option::is_none")]
    pub copy: Option<serde_json::Value>,
}

impl FieldUpdateOperation {
    /// Details of an operation to perform on a field.
    pub fn new() -> FieldUpdateOperation {
        FieldUpdateOperation {
            add: None,
            set: None,
            remove: None,
            edit: None,
            copy: None,
        }
    }
}
