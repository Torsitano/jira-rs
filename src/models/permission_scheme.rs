/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// PermissionScheme : Details of a permission scheme.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PermissionScheme {
    /// The expand options available for the permission scheme.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// The ID of the permission scheme.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The URL of the permission scheme.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The name of the permission scheme. Must be unique.
    #[serde(rename = "name")]
    pub name: String,
    /// A description for the permission scheme.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Box<crate::models::PermissionSchemeScope>>,
    /// The permission scheme to create or update. See [About permission schemes and grants](../api-group-permission-schemes/#about-permission-schemes-and-grants) for more information.
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<crate::models::PermissionGrant>>,
}

impl PermissionScheme {
    /// Details of a permission scheme.
    pub fn new(name: String) -> PermissionScheme {
        PermissionScheme {
            expand: None,
            id: None,
            param_self: None,
            name,
            description: None,
            scope: None,
            permissions: None,
        }
    }
}
