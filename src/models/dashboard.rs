/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// Dashboard : Details of a dashboard.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dashboard {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the dashboard.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the dashboard is selected as a favorite by the user.
    #[serde(rename = "isFavourite", skip_serializing_if = "Option::is_none")]
    pub is_favourite: Option<bool>,
    /// The name of the dashboard.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<crate::models::DashboardOwner>>,
    /// The number of users who have this dashboard as a favorite.
    #[serde(rename = "popularity", skip_serializing_if = "Option::is_none")]
    pub popularity: Option<i64>,
    /// The rank of this dashboard.
    #[serde(rename = "rank", skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    /// The URL of these dashboard details.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The details of any view share permissions for the dashboard.
    #[serde(rename = "sharePermissions", skip_serializing_if = "Option::is_none")]
    pub share_permissions: Option<Vec<crate::models::SharePermission>>,
    /// The details of any edit share permissions for the dashboard.
    #[serde(rename = "editPermissions", skip_serializing_if = "Option::is_none")]
    pub edit_permissions: Option<Vec<crate::models::SharePermission>>,
    /// The automatic refresh interval for the dashboard in milliseconds.
    #[serde(rename = "automaticRefreshMs", skip_serializing_if = "Option::is_none")]
    pub automatic_refresh_ms: Option<i32>,
    /// The URL of the dashboard.
    #[serde(rename = "view", skip_serializing_if = "Option::is_none")]
    pub view: Option<String>,
    /// Whether the current user has permission to edit the dashboard.
    #[serde(rename = "isWritable", skip_serializing_if = "Option::is_none")]
    pub is_writable: Option<bool>,
    /// Whether the current dashboard is system dashboard.
    #[serde(rename = "systemDashboard", skip_serializing_if = "Option::is_none")]
    pub system_dashboard: Option<bool>,
}

impl Dashboard {
    /// Details of a dashboard.
    pub fn new() -> Dashboard {
        Dashboard {
            description: None,
            id: None,
            is_favourite: None,
            name: None,
            owner: None,
            popularity: None,
            rank: None,
            param_self: None,
            share_permissions: None,
            edit_permissions: None,
            automatic_refresh_ms: None,
            view: None,
            is_writable: None,
            system_dashboard: None,
        }
    }
}
