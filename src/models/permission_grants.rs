/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// PermissionGrants : List of permission grants.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PermissionGrants {
    /// Permission grants list.
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<crate::models::PermissionGrant>>,
    /// Expand options that include additional permission grant details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
}

impl PermissionGrants {
    /// List of permission grants.
    pub fn new() -> PermissionGrants {
        PermissionGrants {
            permissions: None,
            expand: None,
        }
    }
}
