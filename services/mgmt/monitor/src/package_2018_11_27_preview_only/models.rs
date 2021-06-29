#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithError {
    pub error: Error,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceInfo {
    pub id: String,
    pub location: String,
    pub properties: workspace_info::Properties,
}
pub mod workspace_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "customerId")]
        pub customer_id: String,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataContainer {
    pub workspace: WorkspaceInfo,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmInsightsOnboardingStatus {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<vm_insights_onboarding_status::Properties>,
}
pub mod vm_insights_onboarding_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "resourceId")]
        pub resource_id: String,
        #[serde(rename = "onboardingStatus")]
        pub onboarding_status: properties::OnboardingStatus,
        #[serde(rename = "dataStatus")]
        pub data_status: properties::DataStatus,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub data: Vec<DataContainer>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum OnboardingStatus {
            #[serde(rename = "onboarded")]
            Onboarded,
            #[serde(rename = "notOnboarded")]
            NotOnboarded,
            #[serde(rename = "unknown")]
            Unknown,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum DataStatus {
            #[serde(rename = "present")]
            Present,
            #[serde(rename = "notPresent")]
            NotPresent,
        }
    }
}
