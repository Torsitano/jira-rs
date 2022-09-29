/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueFieldOption : Details of the options for a select list issue field.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueFieldOption {
    /// The unique identifier for the option. This is only unique within the select field's set of options.
    #[serde(rename = "id")]
    pub id: i64,
    /// The option's name, which is displayed in Jira.
    #[serde(rename = "value")]
    pub value: String,
    /// The properties of the object, as arbitrary key-value pairs. These properties can be searched using JQL, if the extractions (see [Issue Field Option Property Index](https://developer.atlassian.com/cloud/jira/platform/modules/issue-field-option-property-index/)) are defined in the descriptor for the issue field module.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<crate::models::IssueFieldOptionConfiguration>>,
}

impl IssueFieldOption {
    /// Details of the options for a select list issue field.
    pub fn new(id: i64, value: String) -> IssueFieldOption {
        IssueFieldOption {
            id,
            value,
            properties: None,
            config: None,
        }
    }
}
