/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// WorklogVisibility : Details about any restrictions in the visibility of the worklog. Optional when creating or updating a worklog.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WorklogVisibility {
    /// Whether visibility of this item is restricted to a group or role.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    /// The name of the group or role that visibility of this item is restricted to. Please note that the name of a group is mutable, to reliably identify a group use `identifier`.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// The ID of the group or the name of the role that visibility of this item is restricted to.
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

impl WorklogVisibility {
    /// Details about any restrictions in the visibility of the worklog. Optional when creating or updating a worklog.
    pub fn new() -> WorklogVisibility {
        WorklogVisibility {
            r#type: None,
            value: None,
            identifier: None,
        }
    }
}

/// Whether visibility of this item is restricted to a group or role.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "role")]
    Role,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Group
    }
}
