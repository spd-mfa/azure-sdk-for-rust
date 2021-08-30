#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoftwaresList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Software>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Software {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: SoftwareProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoftwareProperties {
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "osPlatform", default, skip_serializing_if = "Option::is_none")]
    pub os_platform: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(rename = "softwareName", default, skip_serializing_if = "Option::is_none")]
    pub software_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "endOfSupportStatus", default, skip_serializing_if = "Option::is_none")]
    pub end_of_support_status: Option<software_properties::EndOfSupportStatus>,
    #[serde(rename = "endOfSupportDate", default, skip_serializing_if = "Option::is_none")]
    pub end_of_support_date: Option<String>,
    #[serde(rename = "numberOfKnownVulnerabilities", default, skip_serializing_if = "Option::is_none")]
    pub number_of_known_vulnerabilities: Option<i32>,
    #[serde(rename = "firstSeenAt", default, skip_serializing_if = "Option::is_none")]
    pub first_seen_at: Option<String>,
}
pub mod software_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EndOfSupportStatus {
        None,
        #[serde(rename = "noLongerSupported")]
        NoLongerSupported,
        #[serde(rename = "versionNoLongerSupported")]
        VersionNoLongerSupported,
        #[serde(rename = "upcomingNoLongerSupported")]
        UpcomingNoLongerSupported,
        #[serde(rename = "upcomingVersionNoLongerSupported")]
        UpcomingVersionNoLongerSupported,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}