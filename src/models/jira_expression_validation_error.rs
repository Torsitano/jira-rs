/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// JiraExpressionValidationError : Details about syntax and type errors. The error details apply to the entire expression, unless the object includes:   *  `line` and `column`  *  `expression`

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JiraExpressionValidationError {
    /// The text line in which the error occurred.
    #[serde(rename = "line", skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,
    /// The text column in which the error occurred.
    #[serde(rename = "column", skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,
    /// The part of the expression in which the error occurred.
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// Details about the error.
    #[serde(rename = "message")]
    pub message: String,
    /// The error type.
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl JiraExpressionValidationError {
    /// Details about syntax and type errors. The error details apply to the entire expression, unless the object includes:   *  `line` and `column`  *  `expression`
    pub fn new(message: String, r#type: RHashType) -> JiraExpressionValidationError {
        JiraExpressionValidationError {
            line: None,
            column: None,
            expression: None,
            message,
            r#type,
        }
    }
}

/// The error type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "syntax")]
    Syntax,
    #[serde(rename = "type")]
    Type,
    #[serde(rename = "other")]
    Other,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Syntax
    }
}
