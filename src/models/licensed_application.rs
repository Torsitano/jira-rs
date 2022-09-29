/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// LicensedApplication : Details about a licensed Jira application.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LicensedApplication {
    /// The ID of the application.
    #[serde(rename = "id")]
    pub id: String,
    /// The licensing plan.
    #[serde(rename = "plan")]
    pub plan: Plan,
}

impl LicensedApplication {
    /// Details about a licensed Jira application.
    pub fn new(id: String, plan: Plan) -> LicensedApplication {
        LicensedApplication { id, plan }
    }
}

/// The licensing plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Plan {
    #[serde(rename = "UNLICENSED")]
    Unlicensed,
    #[serde(rename = "FREE")]
    Free,
    #[serde(rename = "PAID")]
    Paid,
}

impl Default for Plan {
    fn default() -> Plan {
        Self::Unlicensed
    }
}
