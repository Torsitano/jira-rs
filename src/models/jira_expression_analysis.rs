/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// JiraExpressionAnalysis : Details about the analysed Jira expression.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JiraExpressionAnalysis {
    /// The analysed expression.
    #[serde(rename = "expression")]
    pub expression: String,
    /// A list of validation errors. Not included if the expression is valid.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::JiraExpressionValidationError>>,
    /// Whether the expression is valid and the interpreter will evaluate it. Note that the expression may fail at runtime (for example, if it executes too many expensive operations).
    #[serde(rename = "valid")]
    pub valid: bool,
    /// EXPERIMENTAL. The inferred type of the expression.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "complexity", skip_serializing_if = "Option::is_none")]
    pub complexity: Option<Box<crate::models::JiraExpressionComplexity>>,
}

impl JiraExpressionAnalysis {
    /// Details about the analysed Jira expression.
    pub fn new(expression: String, valid: bool) -> JiraExpressionAnalysis {
        JiraExpressionAnalysis {
            expression,
            errors: None,
            valid,
            r#type: None,
            complexity: None,
        }
    }
}
