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
pub struct SearchRequestBean {
    /// A [JQL](https://confluence.atlassian.com/x/egORLQ) expression.
    #[serde(rename = "jql", skip_serializing_if = "Option::is_none")]
    pub jql: Option<String>,
    /// The index of the first item to return in the page of results (page offset). The base index is `0`.
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i32>,
    /// The maximum number of items to return per page.
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// A list of fields to return for each issue, use it to retrieve a subset of fields. This parameter accepts a comma-separated list. Expand options include:   *  `*all` Returns all fields.  *  `*navigable` Returns navigable fields.  *  Any issue field, prefixed with a minus to exclude.  The default is `*navigable`.  Examples:   *  `summary,comment` Returns the summary and comments fields only.  *  `-description` Returns all navigable (default) fields except description.  *  `*all,-comment` Returns all fields except comments.  Multiple `fields` parameters can be included in a request.  Note: All navigable fields are returned by default. This differs from [GET issue](#api-rest-api-3-issue-issueIdOrKey-get) where the default is all fields.
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    /// Determines how to validate the JQL query and treat the validation results. Supported values:   *  `strict` Returns a 400 response code if any errors are found, along with a list of all errors (and warnings).  *  `warn` Returns all errors as warnings.  *  `none` No validation is performed.  *  `true` *Deprecated* A legacy synonym for `strict`.  *  `false` *Deprecated* A legacy synonym for `warn`.  The default is `strict`.  Note: If the JQL is not correctly formed a 400 response code is returned, regardless of the `validateQuery` value.
    #[serde(rename = "validateQuery", skip_serializing_if = "Option::is_none")]
    pub validate_query: Option<ValidateQuery>,
    /// Use [expand](em>#expansion) to include additional information about issues in the response. Note that, unlike the majority of instances where `expand` is specified, `expand` is defined as a list of values. The expand options are:   *  `renderedFields` Returns field values rendered in HTML format.  *  `names` Returns the display name of each field.  *  `schema` Returns the schema describing a field type.  *  `transitions` Returns all possible transitions for the issue.  *  `operations` Returns all possible operations for the issue.  *  `editmeta` Returns information about how each field can be edited.  *  `changelog` Returns a list of recent updates to an issue, sorted by date, starting from the most recent.  *  `versionedRepresentations` Instead of `fields`, returns `versionedRepresentations` a JSON array containing each version of a field's value, with the highest numbered item representing the most recent version.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
    /// A list of up to 5 issue properties to include in the results. This parameter accepts a comma-separated list.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<String>>,
    /// Reference fields by their key (rather than ID). The default is `false`.
    #[serde(rename = "fieldsByKeys", skip_serializing_if = "Option::is_none")]
    pub fields_by_keys: Option<bool>,
}

impl SearchRequestBean {
    pub fn new() -> SearchRequestBean {
        SearchRequestBean {
            jql: None,
            start_at: None,
            max_results: None,
            fields: None,
            validate_query: None,
            expand: None,
            properties: None,
            fields_by_keys: None,
        }
    }
}

/// Determines how to validate the JQL query and treat the validation results. Supported values:   *  `strict` Returns a 400 response code if any errors are found, along with a list of all errors (and warnings).  *  `warn` Returns all errors as warnings.  *  `none` No validation is performed.  *  `true` *Deprecated* A legacy synonym for `strict`.  *  `false` *Deprecated* A legacy synonym for `warn`.  The default is `strict`.  Note: If the JQL is not correctly formed a 400 response code is returned, regardless of the `validateQuery` value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ValidateQuery {
    #[serde(rename = "strict")]
    Strict,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
}

impl Default for ValidateQuery {
    fn default() -> ValidateQuery {
        Self::Strict
    }
}
