/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceManagementNavigationInfo {
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i64>,
    #[serde(rename = "queueName", skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
    #[serde(rename = "queueCategory", skip_serializing_if = "Option::is_none")]
    pub queue_category: Option<String>,
}

impl ServiceManagementNavigationInfo {
    pub fn new() -> ServiceManagementNavigationInfo {
        ServiceManagementNavigationInfo {
            queue_id: None,
            queue_name: None,
            queue_category: None,
        }
    }
}
