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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: SkuName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Free,
        Standard,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SkuName {
    F0,
    S1,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "sdk")]
    Sdk,
    #[serde(rename = "designer")]
    Designer,
    #[serde(rename = "bot")]
    Bot,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "azurebot")]
    Azurebot,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bot {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BotProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BotProperties {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    pub endpoint: String,
    #[serde(rename = "endpointVersion", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_version: Option<String>,
    #[serde(rename = "msaAppId")]
    pub msa_app_id: String,
    #[serde(rename = "configuredChannels", default, skip_serializing_if = "Vec::is_empty")]
    pub configured_channels: Vec<String>,
    #[serde(rename = "enabledChannels", default, skip_serializing_if = "Vec::is_empty")]
    pub enabled_channels: Vec<String>,
    #[serde(rename = "developerAppInsightKey", default, skip_serializing_if = "Option::is_none")]
    pub developer_app_insight_key: Option<String>,
    #[serde(rename = "developerAppInsightsApiKey", default, skip_serializing_if = "Option::is_none")]
    pub developer_app_insights_api_key: Option<String>,
    #[serde(rename = "developerAppInsightsApplicationId", default, skip_serializing_if = "Option::is_none")]
    pub developer_app_insights_application_id: Option<String>,
    #[serde(rename = "luisAppIds", default, skip_serializing_if = "Vec::is_empty")]
    pub luis_app_ids: Vec<String>,
    #[serde(rename = "luisKey", default, skip_serializing_if = "Option::is_none")]
    pub luis_key: Option<String>,
    #[serde(rename = "isCmekEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_cmek_enabled: Option<bool>,
    #[serde(rename = "cmekKeyVaultUrl", default, skip_serializing_if = "Option::is_none")]
    pub cmek_key_vault_url: Option<String>,
    #[serde(rename = "isIsolated", default, skip_serializing_if = "Option::is_none")]
    pub is_isolated: Option<bool>,
    #[serde(rename = "schemaTransformationVersion", default, skip_serializing_if = "Option::is_none")]
    pub schema_transformation_version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BotResponseList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Bot>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BotChannel {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Channel>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    #[serde(rename = "channelName")]
    pub channel_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlexaChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlexaChannelProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlexaChannelProperties {
    #[serde(rename = "alexaSkillId")]
    pub alexa_skill_id: String,
    #[serde(rename = "urlFragment", default, skip_serializing_if = "Option::is_none")]
    pub url_fragment: Option<String>,
    #[serde(rename = "serviceEndpointUri", default, skip_serializing_if = "Option::is_none")]
    pub service_endpoint_uri: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacebookChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FacebookChannelProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacebookChannelProperties {
    #[serde(rename = "verifyToken", default, skip_serializing_if = "Option::is_none")]
    pub verify_token: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pages: Vec<FacebookPage>,
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "appSecret", default, skip_serializing_if = "Option::is_none")]
    pub app_secret: Option<String>,
    #[serde(rename = "callbackUrl", default, skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacebookPage {
    pub id: String,
    #[serde(rename = "accessToken", default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EmailChannelProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailChannelProperties {
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MsTeamsChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MsTeamsChannelProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MsTeamsChannelProperties {
    #[serde(rename = "enableCalling", default, skip_serializing_if = "Option::is_none")]
    pub enable_calling: Option<bool>,
    #[serde(rename = "callingWebHook", default, skip_serializing_if = "Option::is_none")]
    pub calling_web_hook: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkypeChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SkypeChannelProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkypeChannelProperties {
    #[serde(rename = "enableMessaging", default, skip_serializing_if = "Option::is_none")]
    pub enable_messaging: Option<bool>,
    #[serde(rename = "enableMediaCards", default, skip_serializing_if = "Option::is_none")]
    pub enable_media_cards: Option<bool>,
    #[serde(rename = "enableVideo", default, skip_serializing_if = "Option::is_none")]
    pub enable_video: Option<bool>,
    #[serde(rename = "enableCalling", default, skip_serializing_if = "Option::is_none")]
    pub enable_calling: Option<bool>,
    #[serde(rename = "enableScreenSharing", default, skip_serializing_if = "Option::is_none")]
    pub enable_screen_sharing: Option<bool>,
    #[serde(rename = "enableGroups", default, skip_serializing_if = "Option::is_none")]
    pub enable_groups: Option<bool>,
    #[serde(rename = "groupsMode", default, skip_serializing_if = "Option::is_none")]
    pub groups_mode: Option<String>,
    #[serde(rename = "callingWebHook", default, skip_serializing_if = "Option::is_none")]
    pub calling_web_hook: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KikChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<KikChannelProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KikChannelProperties {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "apiKey", default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(rename = "isValidated", default, skip_serializing_if = "Option::is_none")]
    pub is_validated: Option<bool>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebChatChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebChatChannelProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebChatChannelProperties {
    #[serde(rename = "webChatEmbedCode", default, skip_serializing_if = "Option::is_none")]
    pub web_chat_embed_code: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sites: Vec<WebChatSite>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectLineChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DirectLineChannelProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectLineChannelProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sites: Vec<DirectLineSite>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TelegramChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TelegramChannelProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TelegramChannelProperties {
    #[serde(rename = "accessToken", default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "isValidated", default, skip_serializing_if = "Option::is_none")]
    pub is_validated: Option<bool>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SmsChannelProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsChannelProperties {
    pub phone: String,
    #[serde(rename = "accountSID")]
    pub account_sid: String,
    #[serde(rename = "authToken", default, skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(rename = "isValidated", default, skip_serializing_if = "Option::is_none")]
    pub is_validated: Option<bool>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlackChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SlackChannelProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlackChannelProperties {
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "verificationToken", default, skip_serializing_if = "Option::is_none")]
    pub verification_token: Option<String>,
    #[serde(rename = "landingPageUrl", default, skip_serializing_if = "Option::is_none")]
    pub landing_page_url: Option<String>,
    #[serde(rename = "redirectAction", default, skip_serializing_if = "Option::is_none")]
    pub redirect_action: Option<String>,
    #[serde(rename = "lastSubmissionId", default, skip_serializing_if = "Option::is_none")]
    pub last_submission_id: Option<String>,
    #[serde(rename = "registerBeforeOAuthFlow", default, skip_serializing_if = "Option::is_none")]
    pub register_before_o_auth_flow: Option<bool>,
    #[serde(rename = "isValidated", default, skip_serializing_if = "Option::is_none")]
    pub is_validated: Option<bool>,
    #[serde(rename = "signingSecret", default, skip_serializing_if = "Option::is_none")]
    pub signing_secret: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LineChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LineChannelProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LineChannelProperties {
    #[serde(rename = "lineRegistrations")]
    pub line_registrations: Vec<LineRegistration>,
    #[serde(rename = "callbackUrl", default, skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[serde(rename = "isValidated", default, skip_serializing_if = "Option::is_none")]
    pub is_validated: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LineRegistration {
    #[serde(rename = "generatedId", default, skip_serializing_if = "Option::is_none")]
    pub generated_id: Option<String>,
    #[serde(rename = "channelSecret", default, skip_serializing_if = "Option::is_none")]
    pub channel_secret: Option<String>,
    #[serde(rename = "channelAccessToken", default, skip_serializing_if = "Option::is_none")]
    pub channel_access_token: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectLineSpeechChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DirectLineSpeechChannelProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectLineSpeechChannelProperties {
    #[serde(rename = "cognitiveServiceRegion")]
    pub cognitive_service_region: String,
    #[serde(rename = "cognitiveServiceSubscriptionKey")]
    pub cognitive_service_subscription_key: String,
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "customVoiceDeploymentId", default, skip_serializing_if = "Option::is_none")]
    pub custom_voice_deployment_id: Option<String>,
    #[serde(rename = "customSpeechModelId", default, skip_serializing_if = "Option::is_none")]
    pub custom_speech_model_id: Option<String>,
    #[serde(rename = "isDefaultBotForCogSvcAccount", default, skip_serializing_if = "Option::is_none")]
    pub is_default_bot_for_cog_svc_account: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelResponseList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BotChannel>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebChatSite {
    #[serde(rename = "siteId", default, skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    #[serde(rename = "siteName")]
    pub site_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key2: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "enablePreview")]
    pub enable_preview: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectLineSite {
    #[serde(rename = "siteId", default, skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    #[serde(rename = "siteName")]
    pub site_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key2: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "isV1Enabled")]
    pub is_v1_enabled: bool,
    #[serde(rename = "isV3Enabled")]
    pub is_v3_enabled: bool,
    #[serde(rename = "isSecureSiteEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_secure_site_enabled: Option<bool>,
    #[serde(rename = "trustedOrigins", default, skip_serializing_if = "Vec::is_empty")]
    pub trusted_origins: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiteInfo {
    #[serde(rename = "siteName")]
    pub site_name: String,
    pub key: site_info::Key,
}
pub mod site_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Key {
        #[serde(rename = "key1")]
        Key1,
        #[serde(rename = "key2")]
        Key2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionItemName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionSettingParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionSettingProperties {
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "settingId", default, skip_serializing_if = "Option::is_none")]
    pub setting_id: Option<String>,
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
    #[serde(rename = "serviceProviderId", default, skip_serializing_if = "Option::is_none")]
    pub service_provider_id: Option<String>,
    #[serde(rename = "serviceProviderDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub service_provider_display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ConnectionSettingParameter>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionSetting {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConnectionSettingProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionSettingResponseList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConnectionSetting>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceProviderResponseList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServiceProvider>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceProviderParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "helpUrl", default, skip_serializing_if = "Option::is_none")]
    pub help_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceProviderProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "serviceProviderName", default, skip_serializing_if = "Option::is_none")]
    pub service_provider_name: Option<String>,
    #[serde(rename = "devPortalUrl", default, skip_serializing_if = "Option::is_none")]
    pub dev_portal_url: Option<String>,
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ServiceProviderParameter>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceProvider {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceProviderProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationEntityListResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationEntity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
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
pub struct CheckNameAvailabilityRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityResponseBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostSettingsResponse {
    #[serde(rename = "OAuthUrl", default, skip_serializing_if = "Option::is_none")]
    pub o_auth_url: Option<String>,
    #[serde(rename = "ToBotFromChannelOpenIdMetadataUrl", default, skip_serializing_if = "Option::is_none")]
    pub to_bot_from_channel_open_id_metadata_url: Option<String>,
    #[serde(rename = "ToBotFromChannelTokenIssuer", default, skip_serializing_if = "Option::is_none")]
    pub to_bot_from_channel_token_issuer: Option<String>,
    #[serde(rename = "ToBotFromEmulatorOpenIdMetadataUrl", default, skip_serializing_if = "Option::is_none")]
    pub to_bot_from_emulator_open_id_metadata_url: Option<String>,
    #[serde(rename = "ToChannelFromBotLoginUrl", default, skip_serializing_if = "Option::is_none")]
    pub to_channel_from_bot_login_url: Option<String>,
    #[serde(rename = "ToChannelFromBotOAuthScope", default, skip_serializing_if = "Option::is_none")]
    pub to_channel_from_bot_o_auth_scope: Option<String>,
    #[serde(rename = "ValidateAuthority", default, skip_serializing_if = "Option::is_none")]
    pub validate_authority: Option<bool>,
    #[serde(rename = "BotOpenIdMetadata", default, skip_serializing_if = "Option::is_none")]
    pub bot_open_id_metadata: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationResultsDescription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<operation_results_description::Status>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}
pub mod operation_results_description {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Canceled,
        Succeeded,
        Failed,
        Requested,
        Running,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub private_link_resource_base: PrivateLinkResourceBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpoint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnectionState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "actionRequired", default, skip_serializing_if = "Option::is_none")]
    pub action_required: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Deleting,
    Failed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub private_link_resource_base: PrivateLinkResourceBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceProperties {
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
