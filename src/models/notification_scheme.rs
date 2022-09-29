/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// NotificationScheme : Details about a notification scheme.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NotificationScheme {
    /// Expand options that include additional notification scheme details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// The ID of the notification scheme.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The name of the notification scheme.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the notification scheme.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The notification events and associated recipients.
    #[serde(
        rename = "notificationSchemeEvents",
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_scheme_events: Option<Vec<crate::models::NotificationSchemeEvent>>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Box<crate::models::NotificationSchemeScope>>,
}

impl NotificationScheme {
    /// Details about a notification scheme.
    pub fn new() -> NotificationScheme {
        NotificationScheme {
            expand: None,
            id: None,
            param_self: None,
            name: None,
            description: None,
            notification_scheme_events: None,
            scope: None,
        }
    }
}
