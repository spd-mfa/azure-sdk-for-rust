#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyEventsQueryResults {
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicyEvent>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyEvent {
    #[serde(rename = "@odata.id", default, skip_serializing_if = "Option::is_none")]
    pub odata_id: Option<String>,
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[serde(rename = "effectiveParameters", default, skip_serializing_if = "Option::is_none")]
    pub effective_parameters: Option<String>,
    #[serde(rename = "isCompliant", default, skip_serializing_if = "Option::is_none")]
    pub is_compliant: Option<bool>,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[serde(rename = "resourceTags", default, skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<String>,
    #[serde(rename = "policyAssignmentName", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_name: Option<String>,
    #[serde(rename = "policyAssignmentOwner", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_owner: Option<String>,
    #[serde(rename = "policyAssignmentParameters", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_parameters: Option<String>,
    #[serde(rename = "policyAssignmentScope", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_scope: Option<String>,
    #[serde(rename = "policyDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_name: Option<String>,
    #[serde(rename = "policyDefinitionAction", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_action: Option<String>,
    #[serde(rename = "policyDefinitionCategory", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_category: Option<String>,
    #[serde(rename = "policySetDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_id: Option<String>,
    #[serde(rename = "policySetDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_name: Option<String>,
    #[serde(rename = "policySetDefinitionOwner", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_owner: Option<String>,
    #[serde(rename = "policySetDefinitionCategory", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_category: Option<String>,
    #[serde(rename = "policySetDefinitionParameters", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_parameters: Option<String>,
    #[serde(rename = "managementGroupIds", default, skip_serializing_if = "Option::is_none")]
    pub management_group_ids: Option<String>,
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "principalOid", default, skip_serializing_if = "Option::is_none")]
    pub principal_oid: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryFailure {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<query_failure::Error>,
}
pub mod query_failure {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Error {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataDocument {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyStatesQueryResults {
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicyState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyState {
    #[serde(rename = "@odata.id", default, skip_serializing_if = "Option::is_none")]
    pub odata_id: Option<String>,
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[serde(rename = "effectiveParameters", default, skip_serializing_if = "Option::is_none")]
    pub effective_parameters: Option<String>,
    #[serde(rename = "isCompliant", default, skip_serializing_if = "Option::is_none")]
    pub is_compliant: Option<bool>,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[serde(rename = "resourceTags", default, skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<String>,
    #[serde(rename = "policyAssignmentName", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_name: Option<String>,
    #[serde(rename = "policyAssignmentOwner", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_owner: Option<String>,
    #[serde(rename = "policyAssignmentParameters", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_parameters: Option<String>,
    #[serde(rename = "policyAssignmentScope", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_scope: Option<String>,
    #[serde(rename = "policyDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_name: Option<String>,
    #[serde(rename = "policyDefinitionAction", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_action: Option<String>,
    #[serde(rename = "policyDefinitionCategory", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_category: Option<String>,
    #[serde(rename = "policySetDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_id: Option<String>,
    #[serde(rename = "policySetDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_name: Option<String>,
    #[serde(rename = "policySetDefinitionOwner", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_owner: Option<String>,
    #[serde(rename = "policySetDefinitionCategory", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_category: Option<String>,
    #[serde(rename = "policySetDefinitionParameters", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_parameters: Option<String>,
    #[serde(rename = "managementGroupIds", default, skip_serializing_if = "Option::is_none")]
    pub management_group_ids: Option<String>,
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SummarizeResults {
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Summary>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Summary {
    #[serde(rename = "@odata.id", default, skip_serializing_if = "Option::is_none")]
    pub odata_id: Option<String>,
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<SummaryResults>,
    #[serde(rename = "policyAssignments", default, skip_serializing_if = "Vec::is_empty")]
    pub policy_assignments: Vec<PolicyAssignmentSummary>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SummaryResults {
    #[serde(rename = "queryResultsUri", default, skip_serializing_if = "Option::is_none")]
    pub query_results_uri: Option<String>,
    #[serde(rename = "nonCompliantResources", default, skip_serializing_if = "Option::is_none")]
    pub non_compliant_resources: Option<i32>,
    #[serde(rename = "nonCompliantPolicies", default, skip_serializing_if = "Option::is_none")]
    pub non_compliant_policies: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyAssignmentSummary {
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[serde(rename = "policySetDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<SummaryResults>,
    #[serde(rename = "policyDefinitions", default, skip_serializing_if = "Vec::is_empty")]
    pub policy_definitions: Vec<PolicyDefinitionSummary>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyDefinitionSummary {
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<SummaryResults>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsListResults {
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
