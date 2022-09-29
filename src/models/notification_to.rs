/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// NotificationTo : The recipients of the email notification for the issue.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NotificationTo {
    /// Whether the notification should be sent to the issue's reporter.
    #[serde(rename = "reporter", skip_serializing_if = "Option::is_none")]
    pub reporter: Option<bool>,
    /// Whether the notification should be sent to the issue's assignees.
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<bool>,
    /// Whether the notification should be sent to the issue's watchers.
    #[serde(rename = "watchers", skip_serializing_if = "Option::is_none")]
    pub watchers: Option<bool>,
    /// Whether the notification should be sent to the issue's voters.
    #[serde(rename = "voters", skip_serializing_if = "Option::is_none")]
    pub voters: Option<bool>,
    /// List of users to receive the notification.
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<crate::models::UserDetails>>,
    /// List of groups to receive the notification.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<crate::models::GroupName>>,
    /// List of groupIds to receive the notification.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
}

impl NotificationTo {
    /// The recipients of the email notification for the issue.
    pub fn new() -> NotificationTo {
        NotificationTo {
            reporter: None,
            assignee: None,
            watchers: None,
            voters: None,
            users: None,
            groups: None,
            group_ids: None,
        }
    }
}
