/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// JqlQueryClauseTimePredicate : A time predicate for a temporal JQL clause.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JqlQueryClauseTimePredicate {
    /// The operator between the field and the operand.
    #[serde(rename = "operator")]
    pub operator: Operator,
    #[serde(rename = "operand")]
    pub operand: Box<crate::models::JqlQueryClauseOperand>,
}

impl JqlQueryClauseTimePredicate {
    /// A time predicate for a temporal JQL clause.
    pub fn new(
        operator: Operator,
        operand: crate::models::JqlQueryClauseOperand,
    ) -> JqlQueryClauseTimePredicate {
        JqlQueryClauseTimePredicate {
            operator,
            operand: Box::new(operand),
        }
    }
}

/// The operator between the field and the operand.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "before")]
    Before,
    #[serde(rename = "after")]
    After,
    #[serde(rename = "from")]
    From,
    #[serde(rename = "to")]
    To,
    #[serde(rename = "on")]
    On,
    #[serde(rename = "during")]
    During,
    #[serde(rename = "by")]
    By,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::Before
    }
}
