/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectComponent : Details about a project component.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectComponent {
    /// The URL of the component.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The unique identifier for the component.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The unique name for the component in the project. Required when creating a component. Optional when updating a component. The maximum length is 255 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description for the component. Optional when creating or updating a component.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lead", skip_serializing_if = "Option::is_none")]
    pub lead: Option<Box<crate::models::ProjectComponentLead>>,
    /// This property is no longer available and will be removed from the documentation soon. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details.
    #[serde(rename = "leadUserName", skip_serializing_if = "Option::is_none")]
    pub lead_user_name: Option<String>,
    /// The accountId of the component's lead user. The accountId uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*.
    #[serde(rename = "leadAccountId", skip_serializing_if = "Option::is_none")]
    pub lead_account_id: Option<String>,
    /// The nominal user type used to determine the assignee for issues created with this component. See `realAssigneeType` for details on how the type of the user, and hence the user, assigned to issues is determined. Can take the following values:   *  `PROJECT_LEAD` the assignee to any issues created with this component is nominally the lead for the project the component is in.  *  `COMPONENT_LEAD` the assignee to any issues created with this component is nominally the lead for the component.  *  `UNASSIGNED` an assignee is not set for issues created with this component.  *  `PROJECT_DEFAULT` the assignee to any issues created with this component is nominally the default assignee for the project that the component is in.  Default value: `PROJECT_DEFAULT`.   Optional when creating or updating a component.
    #[serde(rename = "assigneeType", skip_serializing_if = "Option::is_none")]
    pub assignee_type: Option<AssigneeType>,
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Box<crate::models::ProjectComponentAssignee>>,
    /// The type of the assignee that is assigned to issues created with this component, when an assignee cannot be set from the `assigneeType`. For example, `assigneeType` is set to `COMPONENT_LEAD` but no component lead is set. This property is set to one of the following values:   *  `PROJECT_LEAD` when `assigneeType` is `PROJECT_LEAD` and the project lead has permission to be assigned issues in the project that the component is in.  *  `COMPONENT_LEAD` when `assignee`Type is `COMPONENT_LEAD` and the component lead has permission to be assigned issues in the project that the component is in.  *  `UNASSIGNED` when `assigneeType` is `UNASSIGNED` and Jira is configured to allow unassigned issues.  *  `PROJECT_DEFAULT` when none of the preceding cases are true.
    #[serde(rename = "realAssigneeType", skip_serializing_if = "Option::is_none")]
    pub real_assignee_type: Option<RealAssigneeType>,
    #[serde(rename = "realAssignee", skip_serializing_if = "Option::is_none")]
    pub real_assignee: Option<Box<crate::models::ProjectComponentRealAssignee>>,
    /// Whether a user is associated with `assigneeType`. For example, if the `assigneeType` is set to `COMPONENT_LEAD` but the component lead is not set, then `false` is returned.
    #[serde(
        rename = "isAssigneeTypeValid",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_assignee_type_valid: Option<bool>,
    /// The key of the project the component is assigned to. Required when creating a component. Can't be updated.
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// The ID of the project the component is assigned to.
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
}

impl ProjectComponent {
    /// Details about a project component.
    pub fn new() -> ProjectComponent {
        ProjectComponent {
            param_self: None,
            id: None,
            name: None,
            description: None,
            lead: None,
            lead_user_name: None,
            lead_account_id: None,
            assignee_type: None,
            assignee: None,
            real_assignee_type: None,
            real_assignee: None,
            is_assignee_type_valid: None,
            project: None,
            project_id: None,
        }
    }
}

/// The nominal user type used to determine the assignee for issues created with this component. See `realAssigneeType` for details on how the type of the user, and hence the user, assigned to issues is determined. Can take the following values:   *  `PROJECT_LEAD` the assignee to any issues created with this component is nominally the lead for the project the component is in.  *  `COMPONENT_LEAD` the assignee to any issues created with this component is nominally the lead for the component.  *  `UNASSIGNED` an assignee is not set for issues created with this component.  *  `PROJECT_DEFAULT` the assignee to any issues created with this component is nominally the default assignee for the project that the component is in.  Default value: `PROJECT_DEFAULT`.   Optional when creating or updating a component.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssigneeType {
    #[serde(rename = "PROJECT_DEFAULT")]
    ProjectDefault,
    #[serde(rename = "COMPONENT_LEAD")]
    ComponentLead,
    #[serde(rename = "PROJECT_LEAD")]
    ProjectLead,
    #[serde(rename = "UNASSIGNED")]
    Unassigned,
}

impl Default for AssigneeType {
    fn default() -> AssigneeType {
        Self::ProjectDefault
    }
}
/// The type of the assignee that is assigned to issues created with this component, when an assignee cannot be set from the `assigneeType`. For example, `assigneeType` is set to `COMPONENT_LEAD` but no component lead is set. This property is set to one of the following values:   *  `PROJECT_LEAD` when `assigneeType` is `PROJECT_LEAD` and the project lead has permission to be assigned issues in the project that the component is in.  *  `COMPONENT_LEAD` when `assignee`Type is `COMPONENT_LEAD` and the component lead has permission to be assigned issues in the project that the component is in.  *  `UNASSIGNED` when `assigneeType` is `UNASSIGNED` and Jira is configured to allow unassigned issues.  *  `PROJECT_DEFAULT` when none of the preceding cases are true.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RealAssigneeType {
    #[serde(rename = "PROJECT_DEFAULT")]
    ProjectDefault,
    #[serde(rename = "COMPONENT_LEAD")]
    ComponentLead,
    #[serde(rename = "PROJECT_LEAD")]
    ProjectLead,
    #[serde(rename = "UNASSIGNED")]
    Unassigned,
}

impl Default for RealAssigneeType {
    fn default() -> RealAssigneeType {
        Self::ProjectDefault
    }
}
