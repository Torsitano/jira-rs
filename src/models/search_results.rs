/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// SearchResults : The result of a JQL search.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SearchResults {
    /// Expand options that include additional search result details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// The index of the first item returned on the page.
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i32>,
    /// The maximum number of results that could be on the page.
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The number of results on the page.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// The list of issues found by the search.
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<crate::models::IssueBean>>,
    /// Any warnings related to the JQL query.
    #[serde(rename = "warningMessages", skip_serializing_if = "Option::is_none")]
    pub warning_messages: Option<Vec<String>>,
    /// The ID and name of each field in the search results.
    #[serde(rename = "names", skip_serializing_if = "Option::is_none")]
    pub names: Option<::std::collections::HashMap<String, String>>,
    /// The schema describing the field types in the search results.
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<::std::collections::HashMap<String, crate::models::JsonTypeBean>>,
}

impl SearchResults {
    /// The result of a JQL search.
    pub fn new() -> SearchResults {
        SearchResults {
            expand: None,
            start_at: None,
            max_results: None,
            total: None,
            issues: None,
            warning_messages: None,
            names: None,
            schema: None,
        }
    }
}
