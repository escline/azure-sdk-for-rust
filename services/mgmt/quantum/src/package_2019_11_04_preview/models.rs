#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<QuantumWorkspace>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfferingsListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProviderDescription>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuantumWorkspace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceResourceProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<quantum_workspace::Identity>,
}
pub mod quantum_workspace {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Identity {
        #[serde(rename = "principalId", skip_serializing)]
        pub principal_id: Option<String>,
        #[serde(rename = "tenantId", skip_serializing)]
        pub tenant_id: Option<String>,
        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        pub type_: Option<identity::Type>,
    }
    pub mod identity {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Type {
            SystemAssigned,
            None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagsObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceResourceProperties {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub providers: Vec<Provider>,
    #[serde(skip_serializing)]
    pub usable: Option<workspace_resource_properties::Usable>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<workspace_resource_properties::ProvisioningState>,
    #[serde(rename = "storageAccount", skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<String>,
    #[serde(rename = "endpointUri", skip_serializing)]
    pub endpoint_uri: Option<String>,
}
pub mod workspace_resource_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Usable {
        Yes,
        No,
        Partial,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        ProviderLaunching,
        ProviderUpdating,
        ProviderDeleting,
        ProviderProvisioning,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProviderProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderProperties {
    #[serde(skip_serializing)]
    pub description: Option<String>,
    #[serde(rename = "providerType", skip_serializing)]
    pub provider_type: Option<String>,
    #[serde(skip_serializing)]
    pub company: Option<String>,
    #[serde(rename = "defaultEndpoint", skip_serializing)]
    pub default_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aad: Option<provider_properties::Aad>,
    #[serde(rename = "managedApplication", skip_serializing_if = "Option::is_none")]
    pub managed_application: Option<provider_properties::ManagedApplication>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<TargetDescription>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub skus: Vec<SkuDescription>,
    #[serde(rename = "quotaDimensions", skip_serializing_if = "Vec::is_empty")]
    pub quota_dimensions: Vec<QuotaDimension>,
    #[serde(rename = "pricingDimensions", skip_serializing_if = "Vec::is_empty")]
    pub pricing_dimensions: Vec<PricingDimension>,
}
pub mod provider_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Aad {
        #[serde(rename = "applicationId", skip_serializing)]
        pub application_id: Option<String>,
        #[serde(rename = "tenantId", skip_serializing)]
        pub tenant_id: Option<String>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ManagedApplication {
        #[serde(rename = "publisherId", skip_serializing)]
        pub publisher_id: Option<String>,
        #[serde(rename = "offerId", skip_serializing)]
        pub offer_id: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "acceptedDataFormats", skip_serializing_if = "Vec::is_empty")]
    pub accepted_data_formats: Vec<String>,
    #[serde(rename = "acceptedContentEncodings", skip_serializing_if = "Vec::is_empty")]
    pub accepted_content_encodings: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<String>,
    #[serde(rename = "quotaDimensions", skip_serializing_if = "Vec::is_empty")]
    pub quota_dimensions: Vec<QuotaDimension>,
    #[serde(rename = "pricingDetails", skip_serializing_if = "Vec::is_empty")]
    pub pricing_details: Vec<PricingDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaDimension {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "unitPlural", skip_serializing_if = "Option::is_none")]
    pub unit_plural: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PricingDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PricingDimension {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Provider {
    #[serde(rename = "providerId", skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    #[serde(rename = "providerSku", skip_serializing_if = "Option::is_none")]
    pub provider_sku: Option<String>,
    #[serde(rename = "instanceUri", skip_serializing_if = "Option::is_none")]
    pub instance_uri: Option<String>,
    #[serde(rename = "applicationName", skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<provider::ProvisioningState>,
    #[serde(rename = "resourceUsageId", skip_serializing_if = "Option::is_none")]
    pub resource_usage_id: Option<String>,
}
pub mod provider {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Launching,
        Updating,
        Deleting,
        Deleted,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isDataAction", skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsList {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ErrorDetail>,
    #[serde(rename = "additionalInfo", skip_serializing)]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub info: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}