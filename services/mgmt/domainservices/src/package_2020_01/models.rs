#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainService {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DomainServiceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainServiceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DomainService>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainServiceProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "deploymentId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "syncOwner", default, skip_serializing_if = "Option::is_none")]
    pub sync_owner: Option<String>,
    #[serde(rename = "replicaSets", default, skip_serializing_if = "Vec::is_empty")]
    pub replica_sets: Vec<ReplicaSet>,
    #[serde(rename = "ldapsSettings", default, skip_serializing_if = "Option::is_none")]
    pub ldaps_settings: Option<LdapsSettings>,
    #[serde(rename = "resourceForestSettings", default, skip_serializing_if = "Option::is_none")]
    pub resource_forest_settings: Option<ResourceForestSettings>,
    #[serde(rename = "domainSecuritySettings", default, skip_serializing_if = "Option::is_none")]
    pub domain_security_settings: Option<DomainSecuritySettings>,
    #[serde(rename = "domainConfigurationType", default, skip_serializing_if = "Option::is_none")]
    pub domain_configuration_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "filteredSync", default, skip_serializing_if = "Option::is_none")]
    pub filtered_sync: Option<domain_service_properties::FilteredSync>,
    #[serde(rename = "notificationSettings", default, skip_serializing_if = "Option::is_none")]
    pub notification_settings: Option<NotificationSettings>,
    #[serde(rename = "migrationProperties", default, skip_serializing_if = "Option::is_none")]
    pub migration_properties: Option<MigrationProperties>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
pub mod domain_service_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FilteredSync {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicaSet {
    #[serde(rename = "replicaSetId", default, skip_serializing_if = "Option::is_none")]
    pub replica_set_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "vnetSiteId", default, skip_serializing_if = "Option::is_none")]
    pub vnet_site_id: Option<String>,
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "domainControllerIpAddress", default, skip_serializing_if = "Vec::is_empty")]
    pub domain_controller_ip_address: Vec<String>,
    #[serde(rename = "externalAccessIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub external_access_ip_address: Option<String>,
    #[serde(rename = "serviceStatus", default, skip_serializing_if = "Option::is_none")]
    pub service_status: Option<String>,
    #[serde(rename = "healthLastEvaluated", default, skip_serializing_if = "Option::is_none")]
    pub health_last_evaluated: Option<String>,
    #[serde(rename = "healthMonitors", default, skip_serializing_if = "Vec::is_empty")]
    pub health_monitors: Vec<HealthMonitor>,
    #[serde(rename = "healthAlerts", default, skip_serializing_if = "Vec::is_empty")]
    pub health_alerts: Vec<HealthAlert>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LdapsSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ldaps: Option<ldaps_settings::Ldaps>,
    #[serde(rename = "pfxCertificate", default, skip_serializing_if = "Option::is_none")]
    pub pfx_certificate: Option<String>,
    #[serde(rename = "pfxCertificatePassword", default, skip_serializing_if = "Option::is_none")]
    pub pfx_certificate_password: Option<String>,
    #[serde(rename = "publicCertificate", default, skip_serializing_if = "Option::is_none")]
    pub public_certificate: Option<String>,
    #[serde(rename = "certificateThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub certificate_thumbprint: Option<String>,
    #[serde(rename = "certificateNotAfter", default, skip_serializing_if = "Option::is_none")]
    pub certificate_not_after: Option<String>,
    #[serde(rename = "externalAccess", default, skip_serializing_if = "Option::is_none")]
    pub external_access: Option<ldaps_settings::ExternalAccess>,
}
pub mod ldaps_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Ldaps {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ExternalAccess {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthMonitor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthAlert {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raised: Option<String>,
    #[serde(rename = "lastDetected", default, skip_serializing_if = "Option::is_none")]
    pub last_detected: Option<String>,
    #[serde(rename = "resolutionUri", default, skip_serializing_if = "Option::is_none")]
    pub resolution_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationSettings {
    #[serde(rename = "notifyGlobalAdmins", default, skip_serializing_if = "Option::is_none")]
    pub notify_global_admins: Option<notification_settings::NotifyGlobalAdmins>,
    #[serde(rename = "notifyDcAdmins", default, skip_serializing_if = "Option::is_none")]
    pub notify_dc_admins: Option<notification_settings::NotifyDcAdmins>,
    #[serde(rename = "additionalRecipients", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_recipients: Vec<String>,
}
pub mod notification_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NotifyGlobalAdmins {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NotifyDcAdmins {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceForestSettings {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub settings: Vec<ForestTrust>,
    #[serde(rename = "resourceForest", default, skip_serializing_if = "Option::is_none")]
    pub resource_forest: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForestTrust {
    #[serde(rename = "trustedDomainFqdn", default, skip_serializing_if = "Option::is_none")]
    pub trusted_domain_fqdn: Option<String>,
    #[serde(rename = "trustDirection", default, skip_serializing_if = "Option::is_none")]
    pub trust_direction: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "remoteDnsIps", default, skip_serializing_if = "Option::is_none")]
    pub remote_dns_ips: Option<String>,
    #[serde(rename = "trustPassword", default, skip_serializing_if = "Option::is_none")]
    pub trust_password: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrationProperties {
    #[serde(rename = "oldSubnetId", default, skip_serializing_if = "Option::is_none")]
    pub old_subnet_id: Option<String>,
    #[serde(rename = "oldVnetSiteId", default, skip_serializing_if = "Option::is_none")]
    pub old_vnet_site_id: Option<String>,
    #[serde(rename = "migrationProgress", default, skip_serializing_if = "Option::is_none")]
    pub migration_progress: Option<MigrationProgress>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrationProgress {
    #[serde(rename = "completionPercentage", default, skip_serializing_if = "Option::is_none")]
    pub completion_percentage: Option<f64>,
    #[serde(rename = "progressMessage", default, skip_serializing_if = "Option::is_none")]
    pub progress_message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainSecuritySettings {
    #[serde(rename = "ntlmV1", default, skip_serializing_if = "Option::is_none")]
    pub ntlm_v1: Option<domain_security_settings::NtlmV1>,
    #[serde(rename = "tlsV1", default, skip_serializing_if = "Option::is_none")]
    pub tls_v1: Option<domain_security_settings::TlsV1>,
    #[serde(rename = "syncNtlmPasswords", default, skip_serializing_if = "Option::is_none")]
    pub sync_ntlm_passwords: Option<domain_security_settings::SyncNtlmPasswords>,
    #[serde(rename = "syncKerberosPasswords", default, skip_serializing_if = "Option::is_none")]
    pub sync_kerberos_passwords: Option<domain_security_settings::SyncKerberosPasswords>,
    #[serde(rename = "syncOnPremPasswords", default, skip_serializing_if = "Option::is_none")]
    pub sync_on_prem_passwords: Option<domain_security_settings::SyncOnPremPasswords>,
}
pub mod domain_security_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NtlmV1 {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TlsV1 {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SyncNtlmPasswords {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SyncKerberosPasswords {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SyncOnPremPasswords {
        Enabled,
        Disabled,
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
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationEntityListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationEntity>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplayInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OuContainerListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OuContainer>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OuContainer {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OuContainerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OuContainerProperties {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "deploymentId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "containerId", default, skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<ContainerAccount>,
    #[serde(rename = "serviceStatus", default, skip_serializing_if = "Option::is_none")]
    pub service_status: Option<String>,
    #[serde(rename = "distinguishedName", default, skip_serializing_if = "Option::is_none")]
    pub distinguished_name: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerAccount {
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
