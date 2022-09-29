/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// FieldsTimetracking : The time tracking of the linked issue.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FieldsTimetracking {
    /// The original estimate of time needed for this issue in readable format.
    #[serde(rename = "originalEstimate", skip_serializing_if = "Option::is_none")]
    pub original_estimate: Option<String>,
    /// The remaining estimate of time needed for this issue in readable format.
    #[serde(rename = "remainingEstimate", skip_serializing_if = "Option::is_none")]
    pub remaining_estimate: Option<String>,
    /// Time worked on this issue in readable format.
    #[serde(rename = "timeSpent", skip_serializing_if = "Option::is_none")]
    pub time_spent: Option<String>,
    /// The original estimate of time needed for this issue in seconds.
    #[serde(
        rename = "originalEstimateSeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_estimate_seconds: Option<i64>,
    /// The remaining estimate of time needed for this issue in seconds.
    #[serde(
        rename = "remainingEstimateSeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub remaining_estimate_seconds: Option<i64>,
    /// Time worked on this issue in seconds.
    #[serde(rename = "timeSpentSeconds", skip_serializing_if = "Option::is_none")]
    pub time_spent_seconds: Option<i64>,
}

impl FieldsTimetracking {
    /// The time tracking of the linked issue.
    pub fn new() -> FieldsTimetracking {
        FieldsTimetracking {
            original_estimate: None,
            remaining_estimate: None,
            time_spent: None,
            original_estimate_seconds: None,
            remaining_estimate_seconds: None,
            time_spent_seconds: None,
        }
    }
}
