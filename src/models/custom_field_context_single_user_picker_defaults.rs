/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldContextSingleUserPickerDefaults : Defaults for a User Picker (single) custom field.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldContextSingleUserPickerDefaults {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The ID of the default user.
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "userFilter")]
    pub user_filter: Box<crate::models::UserFilter>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CustomFieldContextSingleUserPickerDefaults {
    /// Defaults for a User Picker (single) custom field.
    pub fn new(
        context_id: String,
        account_id: String,
        user_filter: crate::models::UserFilter,
        r#type: String,
    ) -> CustomFieldContextSingleUserPickerDefaults {
        CustomFieldContextSingleUserPickerDefaults {
            context_id,
            account_id,
            user_filter: Box::new(user_filter),
            r#type,
        }
    }
}
