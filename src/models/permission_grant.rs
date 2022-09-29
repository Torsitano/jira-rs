/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// PermissionGrant : Details about a permission granted to a user or group.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PermissionGrant {
    /// The ID of the permission granted details.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The URL of the permission granted details.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    #[serde(rename = "holder", skip_serializing_if = "Option::is_none")]
    pub holder: Option<Box<crate::models::PermissionGrantHolder>>,
    /// The permission to grant. This permission can be one of the built-in permissions or a custom permission added by an app. See [Built-in permissions](../api-group-permission-schemes/#built-in-permissions) in *Get all permission schemes* for more information about the built-in permissions. See the [project permission](https://developer.atlassian.com/cloud/jira/platform/modules/project-permission/) and [global permission](https://developer.atlassian.com/cloud/jira/platform/modules/global-permission/) module documentation for more information about custom permissions.
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

impl PermissionGrant {
    /// Details about a permission granted to a user or group.
    pub fn new() -> PermissionGrant {
        PermissionGrant {
            id: None,
            param_self: None,
            holder: None,
            permission: None,
        }
    }
}
